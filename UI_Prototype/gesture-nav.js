document.addEventListener('DOMContentLoaded', () => {
    const fab = document.getElementById('gesture-fab');
    if (!fab) return;

    const container = document.querySelector('.gesture-nav-container');

    // Create radial menu container
    let ringContainer = document.createElement('div');
    ringContainer.id = 'gesture-ring';
    ringContainer.style.cssText = `
        position: absolute;
        top: 50%;
        left: 50%;
        width: 0;
        height: 0;
        z-index: -1;
        pointer-events: none;
        opacity: 0;
        transition: opacity 0.2s;
    `;

    // Define the nodes (semi-circle pointing up)
    const radius = 130;
    const nodes = [
        { id: 'nav-left', icon: '⬅️', label: 'Back', x: -120, y: -20, href: '', action: 'back' },
        { id: 'nav-up-left', icon: '📷', label: 'QR', x: -85, y: -90, href: 'scanner.html', action: 'url' },
        { id: 'nav-up', icon: '🗺️', label: 'Home', x: 0, y: -130, href: 'index.html', action: 'url' },
        { id: 'nav-up-right', icon: '📍', label: 'Map', x: 85, y: -90, href: 'map.html', action: 'url' },
        { id: 'nav-right', icon: '👤', label: 'Profile', x: 120, y: -20, href: 'profile.html', action: 'url' }
    ];

    const nodeEls = [];

    nodes.forEach(node => {
        let el = document.createElement('div');
        el.className = 'nav-node';
        el.style.cssText = `
            position: absolute;
            top: ${node.y}px;
            left: ${node.x}px;
            transform: translate(-50%, -50%) scale(0.5);
            background: rgba(255, 255, 255, 0.95);
            border-radius: 50%;
            width: 48px;
            height: 48px;
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 24px;
            box-shadow: 0 4px 15px rgba(0,0,0,0.15);
            transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
            opacity: 0;
            border: 2px solid transparent;
        `;
        el.innerHTML = node.icon;

        let label = document.createElement('span');
        label.style.cssText = `
            position: absolute;
            bottom: -20px;
            font-size: 11px;
            font-weight: 700;
            color: #333;
            white-space: nowrap;
            background: rgba(255,255,255,0.8);
            padding: 2px 6px;
            border-radius: 6px;
        `;
        label.innerText = node.label;
        el.appendChild(label);

        ringContainer.appendChild(el);
        nodeEls.push({ node, el });
    });

    container.appendChild(ringContainer);

    let startX = 0;
    let startY = 0;
    let isDragging = false;
    let hasMoved = false;

    let activeNodeIndex = -1;
    let clickCount = 0;
    let clickTimer = null;

    fab.addEventListener('pointerdown', (e) => {
        startX = e.clientX;
        startY = e.clientY;
        isDragging = true;
        hasMoved = false;
        activeNodeIndex = -1;

        try { fab.setPointerCapture(e.pointerId); } catch (err) { }

        fab.style.transform = 'scale(0.9)';

        // Show ring
        ringContainer.style.opacity = '1';
        nodeEls.forEach((item, i) => {
            setTimeout(() => {
                if (isDragging) {
                    item.el.style.transform = 'translate(-50%, -50%) scale(1)';
                    item.el.style.opacity = '1';
                }
            }, Math.random() * 50); // slight random delay for nice pop effect
        });
    });

    document.addEventListener('pointermove', (e) => {
        if (!isDragging) return;

        const dx = e.clientX - startX;
        const dy = e.clientY - startY;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist > 10) {
            hasMoved = true;
        }

        const moveX = Math.max(-90, Math.min(90, dx));
        const moveY = Math.max(-90, Math.min(90, dy));
        fab.style.transform = `scale(0.9) translate(${moveX}px, ${moveY}px)`;

        // Check intersection with nodes
        activeNodeIndex = -1;
        nodeEls.forEach((item, i) => {
            const ndx = moveX - item.node.x;
            const ndy = moveY - item.node.y;
            const nodeDist = Math.sqrt(ndx * ndx + ndy * ndy);

            // If the dragged button is close enough to the icon
            if (nodeDist < 45) {
                activeNodeIndex = i;
                item.el.style.transform = 'translate(-50%, -50%) scale(1.2)';
                item.el.style.borderColor = 'var(--accent)';
                item.el.style.boxShadow = '0 6px 20px rgba(255, 112, 67, 0.4)';
            } else {
                item.el.style.transform = 'translate(-50%, -50%) scale(1)';
                item.el.style.borderColor = 'transparent';
                item.el.style.boxShadow = '0 4px 15px rgba(0,0,0,0.15)';
            }
        });
    });

    document.addEventListener('pointerup', (e) => {
        if (!isDragging) return;
        isDragging = false;

        try { fab.releasePointerCapture(e.pointerId); } catch (err) { }

        fab.style.transform = '';

        // Hide ring
        ringContainer.style.opacity = '0';
        nodeEls.forEach(item => {
            item.el.style.transform = 'translate(-50%, -50%) scale(0.5)';
            item.el.style.opacity = '0';
            item.el.style.borderColor = 'transparent';
        });

        // Click logic (Double click for AI)
        if (!hasMoved) {
            clickCount++;
            if (clickCount === 1) {
                clickTimer = setTimeout(() => {
                    clickCount = 0;
                    // Single click does nothing or you can add specific behavior
                }, 250); // Wait 250ms to see if second click comes
            } else if (clickCount === 2) {
                clearTimeout(clickTimer);
                clickCount = 0;
                // Double click action
                location.href = 'search.html';
            }
            return;
        }

        // Drag drop logic
        if (activeNodeIndex !== -1) {
            const target = nodeEls[activeNodeIndex].node;
            setTimeout(() => {
                if (target.action === 'back') {
                    history.back();
                } else {
                    location.href = target.href;
                }
            }, 100);
        }
    });

    fab.addEventListener('touchmove', (e) => {
        if (isDragging) {
            e.preventDefault();
        }
    }, { passive: false });
});
