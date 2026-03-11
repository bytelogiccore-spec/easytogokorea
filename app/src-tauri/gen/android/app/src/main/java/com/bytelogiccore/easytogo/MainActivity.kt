package com.bytelogiccore.easytogo

import android.os.Bundle
import android.util.Log
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import com.google.ar.core.ArCoreApk
import com.google.ar.core.Session
import com.google.ar.core.Config
import com.google.ar.core.Frame
import com.google.ar.core.HitResult
import com.google.ar.core.Plane
import com.google.ar.core.Anchor
import com.google.ar.core.TrackingState

class MainActivity : TauriActivity() {
    private var arSession: Session? = null
    private var arAvailable = false

    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)
        checkArAvailability()
    }

    override fun onResume() {
        super.onResume()
        // WebView JS bridge will be set up by Tauri
    }

    override fun onPause() {
        super.onPause()
        arSession?.pause()
    }

    override fun onDestroy() {
        super.onDestroy()
        arSession?.close()
        arSession = null
    }

    private fun checkArAvailability() {
        val availability = ArCoreApk.getInstance().checkAvailability(this)
        arAvailable = when {
            availability.isTransient -> {
                // Re-check later
                android.os.Handler(mainLooper).postDelayed({ checkArAvailability() }, 200)
                false
            }
            availability.isSupported -> {
                Log.i("EasyToGoAR", "ARCore is supported on this device")
                true
            }
            else -> {
                Log.w("EasyToGoAR", "ARCore is NOT supported on this device")
                false
            }
        }
    }

    /**
     * Create an AR session. Called from Rust via Tauri plugin system.
     * Returns true if session created successfully.
     */
    fun createArSession(): Boolean {
        if (!arAvailable) return false
        return try {
            arSession = Session(this).apply {
                val config = Config(this).apply {
                    planeFindingMode = Config.PlaneFindingMode.HORIZONTAL_AND_VERTICAL
                    updateMode = Config.UpdateMode.LATEST_CAMERA_IMAGE
                    focusMode = Config.FocusMode.AUTO
                }
                configure(config)
                resume()
            }
            Log.i("EasyToGoAR", "AR Session created successfully")
            true
        } catch (e: Exception) {
            Log.e("EasyToGoAR", "Failed to create AR session: ${e.message}")
            false
        }
    }

    /**
     * Perform a hit test at normalized screen coordinates (0..1).
     * Returns anchor info as JSON string, or null if no hit.
     */
    fun hitTest(normalizedX: Float, normalizedY: Float): String? {
        val session = arSession ?: return null
        val frame = session.update()

        if (frame.camera.trackingState != TrackingState.TRACKING) return null

        val hits = frame.hitTest(normalizedX, normalizedY)
        for (hit in hits) {
            val trackable = hit.trackable
            if (trackable is Plane && trackable.isPoseInPolygon(hit.hitPose)) {
                val anchor = hit.createAnchor()
                val pose = anchor.pose
                return """{"x":${pose.tx()},"y":${pose.ty()},"z":${pose.tz()},"qx":${pose.qx()},"qy":${pose.qy()},"qz":${pose.qz()},"qw":${pose.qw()}}"""
            }
        }
        return null
    }

    /**
     * Get current tracking state as JSON.
     */
    fun getTrackingInfo(): String {
        val session = arSession ?: return """{"available":$arAvailable,"tracking":false,"planes":0}"""
        return try {
            val frame = session.update()
            val planes = session.getAllTrackables(Plane::class.java)
                .count { it.trackingState == TrackingState.TRACKING }
            val tracking = frame.camera.trackingState == TrackingState.TRACKING
            """{"available":true,"tracking":$tracking,"planes":$planes}"""
        } catch (e: Exception) {
            """{"available":true,"tracking":false,"planes":0,"error":"${e.message}"}"""
        }
    }

    fun isArAvailable(): Boolean = arAvailable
}
