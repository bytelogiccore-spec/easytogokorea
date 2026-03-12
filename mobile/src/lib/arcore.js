/**
 * ARCore bridge utility for Svelte.
 *
 * On Android, this wraps `window.ArBridge.*` (the @JavascriptInterface).
 * On desktop/web, all methods return safe fallback values.
 *
 * Usage:
 *   import { isNative, startSession, getTrackingState, hitTest } from '$lib/arcore.js';
 */

/** Check if running on Android with the ArBridge injected */
export function isNative() {
  return typeof window !== 'undefined' && typeof window.ArBridge !== 'undefined';
}

/** Check ARCore availability. Returns { available: bool, reason: string } */
export function checkAvailability() {
  if (!isNative()) {
    return { available: false, reason: 'Not running on Android' };
  }
  try {
    return JSON.parse(window.ArBridge.checkAvailability());
  } catch (e) {
    return { available: false, reason: e.message };
  }
}

/** Start an ARCore session. Returns { success: bool, reason: string } */
export function startSession() {
  if (!isNative()) {
    return { success: false, reason: 'Not running on Android' };
  }
  try {
    return JSON.parse(window.ArBridge.startSession());
  } catch (e) {
    return { success: false, reason: e.message };
  }
}

/** Stop the ARCore session */
export function stopSession() {
  if (!isNative()) return { success: true };
  try {
    return JSON.parse(window.ArBridge.stopSession());
  } catch (e) {
    return { success: false, reason: e.message };
  }
}

/**
 * Get current tracking state.
 * Returns { tracking: bool, planes: number, pose?: { tx, ty, tz, qx, qy, qz, qw } }
 */
export function getTrackingState() {
  if (!isNative()) {
    return { tracking: false, planes: 0, reason: 'Not native' };
  }
  try {
    return JSON.parse(window.ArBridge.getTrackingState());
  } catch (e) {
    return { tracking: false, planes: 0, error: e.message };
  }
}

/**
 * Perform a hit test at screen coordinates.
 * Returns { hit: bool, id?: string, pose?: {...}, planeType?: string }
 */
export function hitTest(screenX, screenY) {
  if (!isNative()) {
    return { hit: false, reason: 'Not native' };
  }
  try {
    return JSON.parse(window.ArBridge.hitTest(screenX, screenY));
  } catch (e) {
    return { hit: false, reason: e.message };
  }
}

/** Get all active anchors. Returns array of { id, tracking, pose } */
export function getAnchors() {
  if (!isNative()) return [];
  try {
    return JSON.parse(window.ArBridge.getAnchors());
  } catch (e) {
    return [];
  }
}

/** Remove a specific anchor by ID */
export function removeAnchor(id) {
  if (!isNative()) return false;
  try {
    return window.ArBridge.removeAnchor(id);
  } catch (e) {
    return false;
  }
}

/** Clear all anchors */
export function clearAnchors() {
  if (!isNative()) return;
  try {
    window.ArBridge.clearAnchors();
  } catch (e) {
    // ignore
  }
}
