document.addEventListener("DOMContentLoaded", () => {
    runAnimation();

    document.getElementById('replay-btn').addEventListener('click', () => {
        runAnimation();
    });
});

function runAnimation() {
    const letters = document.querySelectorAll('.bouncy-letter');
    const subText = document.querySelector('.sub-text');
    const airplaneContainer = document.querySelector('#airplane-wrapper');
    const airplane = document.querySelector('.airplane');
    const flightPath = document.querySelector('.flight-path');
    const taegukO = document.querySelector('#taeguk-o');
    const taegukColors = document.querySelector('#taeguk-colors');
    const taegukWhite = document.querySelector('#taeguk-white');
    const clouds = document.querySelectorAll('.clouds path');

    // 1. Reset everything
    airplane.style.animation = 'none';

    letters.forEach(letter => {
        letter.style.animation = 'none';
        letter.style.opacity = '0';
        letter.style.transform = 'none';
    });

    subText.style.transition = 'none';
    subText.style.opacity = '0';
    subText.style.transform = 'translateY(20px) scale(0.9)';
    subText.style.letterSpacing = '4px';

    flightPath.style.transition = 'none';
    const pathTotalLen = flightPath.getTotalLength();
    flightPath.style.strokeDasharray = `${pathTotalLen} ${pathTotalLen}`;
    flightPath.style.strokeDashoffset = pathTotalLen;
    flightPath.style.opacity = '0';

    airplaneContainer.style.opacity = '0';
    airplaneContainer.style.transform = 'none';
    airplane.style.transform = 'scale(0.3) rotate(-20deg)';
    airplane.style.transition = 'none';

    // Reset Taeguk: colors hidden, white visible
    taegukColors.style.transition = 'none';
    taegukColors.setAttribute('opacity', '0');
    taegukWhite.style.transition = 'none';
    taegukWhite.setAttribute('opacity', '1');

    clouds.forEach(cloud => {
        cloud.style.transition = 'none';
        cloud.style.strokeDasharray = '300';
        cloud.style.strokeDashoffset = '300';
    });

    void document.body.offsetWidth;

    // 2. Animate Bouncy Clouds
    setTimeout(() => {
        clouds.forEach((cloud, index) => {
            setTimeout(() => {
                cloud.style.transition = 'stroke-dashoffset 1.5s cubic-bezier(0.34, 1.56, 0.64, 1)';
                cloud.style.strokeDashoffset = '0';
            }, index * 150);
        });
    }, 300);

    // 3. BeeKrafty Distort Reveal for Text
    setTimeout(() => {
        letters.forEach((letter, index) => {
            setTimeout(() => {
                letter.style.animation = 'letterPop 0.8s cubic-bezier(0.34, 1.56, 0.64, 1) forwards';
            }, index * 80);
        });
    }, 600);

    // Punchy Subtitle
    setTimeout(() => {
        subText.style.transition = 'all 1s cubic-bezier(0.34, 1.56, 0.64, 1)';
        subText.style.opacity = '1';
        subText.style.transform = 'translateY(0) scale(1)';
        subText.style.letterSpacing = '14px';
    }, 1200);

    // 4. Flight path and airplane → hits 'O' in 'GO'
    setTimeout(() => {
        flightPath.style.transition = 'stroke-dashoffset 1.5s cubic-bezier(0.45, 0, 0.15, 1), opacity 0.3s ease';
        flightPath.style.opacity = '1';
        flightPath.style.strokeDashoffset = '0';

        airplaneContainer.style.opacity = '1';
        airplane.style.transition = 'transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1)';
        airplane.style.transform = 'scale(1) rotate(0deg)';

        animateAirplaneAlongPath(airplaneContainer, flightPath, 1500, () => {
            // === IMPACT! ===

            // Instantly hide airplane and flight path
            airplaneContainer.style.transition = 'opacity 0.15s ease';
            airplaneContainer.style.opacity = '0';
            flightPath.style.transition = 'opacity 0.3s ease';
            flightPath.style.opacity = '0';

            // Fade out white fill
            taegukWhite.style.transition = 'opacity 0.3s ease';
            taegukWhite.setAttribute('opacity', '0');

            // Fade in taeguk colors
            taegukColors.style.transition = 'opacity 0.3s ease';
            taegukColors.setAttribute('opacity', '1');

            // Spin the whole O on impact
            taegukO.style.transition = 'transform 0.8s cubic-bezier(0.34, 1.56, 0.64, 1)';
            taegukO.style.transform = 'rotate(360deg) scale(1.15)';

            setTimeout(() => {
                taegukO.style.transition = 'transform 0.5s ease';
                taegukO.style.transform = 'rotate(360deg) scale(1)';
            }, 800);
        });
    }, 1400);
}

function animateAirplaneAlongPath(airplaneContainer, path, duration, onComplete) {
    const pathLength = path.getTotalLength();
    let start = null;
    let reqId;

    function step(timestamp) {
        if (!start) start = timestamp;
        const progress = Math.min((timestamp - start) / duration, 1);

        // Easing to start fast and slow down a bit near target
        const easeProgress = progress < 0.5
            ? 2 * progress * progress
            : 1 - Math.pow(-2 * progress + 2, 2) / 2;

        const currentPoint = path.getPointAtLength(easeProgress * pathLength);

        const nextPoint = path.getPointAtLength(Math.min((easeProgress * pathLength) + 1, pathLength));
        let angle = Math.atan2(nextPoint.y - currentPoint.y, nextPoint.x - currentPoint.x) * 180 / Math.PI;

        airplaneContainer.style.transform = `translate(${currentPoint.x}px, ${currentPoint.y}px) rotate(${angle}deg)`;

        if (progress < 1) {
            reqId = window.requestAnimationFrame(step);
        } else {
            window.cancelAnimationFrame(reqId);
            if (onComplete) onComplete();
        }
    }

    reqId = window.requestAnimationFrame(step);
}
