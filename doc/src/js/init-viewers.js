import { OffViewer } from './off-viewer.js';

// Store viewers for potential disposal
const viewers = [];

// Initialize viewers for all .off-viewer divs
document.querySelectorAll('.off-viewer').forEach((div, index) => {
    const src = div.getAttribute('data-src');
    if (!src) {
        console.error(`No data-src attribute found on .off-viewer div at index ${index}`);
        return;
    }
    // Assign a unique ID if none exists
    if (!div.id) {
        div.id = `off-viewer-${index}`;
    }
    console.log(`Initializing viewer for div ${div.id} with src ${src}`);
    const viewer = new OffViewer(div.id, src);
    viewers.push(viewer);
});