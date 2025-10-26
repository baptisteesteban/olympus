import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

export class OffViewer {
    constructor(containerId, offFileUrl, displayEdges = true) {
        this.container = document.getElementById(containerId);
        if (!this.container) {
            console.error(`No element found with ID: ${containerId}`);
            return;
        }
        this.offFileUrl = offFileUrl;
        this.displayEdges = displayEdges
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        this.controls = null;
        this.init();
    }

    init() {
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0xffffff);

        this.camera = new THREE.PerspectiveCamera(
            75,
            this.container.clientWidth / this.container.clientHeight,
            0.1,
            1000
        );
        this.camera.position.set(0, 0, 5);

        this.renderer = new THREE.WebGLRenderer({ antialias: true });
        this.renderer.setSize(this.container.clientWidth, this.container.clientHeight);
        this.container.appendChild(this.renderer.domElement);

        this.controls = new OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = true;
        this.controls.dampingFactor = 0.05;

        const ambientLight = new THREE.AmbientLight(0xbbbbbb, 1);
        this.scene.add(ambientLight);
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
        directionalLight.position.set(1, 1, 1).normalize();
        this.scene.add(directionalLight);

        window.addEventListener('resize', () => this.onWindowResize());

        this.loadOffFile();
        this.animate();
    }

    onWindowResize() {
        this.camera.aspect = this.container.clientWidth / this.container.clientHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(this.container.clientWidth, this.container.clientHeight);
    }

    async loadOffFile() {
        try {
            const response = await fetch(this.offFileUrl);
            if (!response.ok) throw new Error(`Failed to load OFF file: ${response.statusText}`);
            const text = await response.text();
            this.parseOffFile(text);
        } catch (error) {
            console.error('Error loading OFF file:', error);
        }
    }

    parseOffFile(data) {
        const lines = data.trim().split('\n').map(line => line.trim()).filter(line => line && !line.startsWith('#'));
        if (lines[0] !== 'OFF') {
            console.error('Invalid OFF file: Missing OFF header');
            return;
        }

        const [vertexCount, faceCount] = lines[1].split(/\s+/).map(Number);

        const vertices = [];
        for (let i = 2; i < 2 + vertexCount; i++) {
            const coords = lines[i].split(/\s+/).map(Number);
            vertices.push(new THREE.Vector3(coords[0], coords[1], coords[2]));
        }

        const geometry = new THREE.BufferGeometry();
        const positions = [];
        const normals = [];
        const edgePositions = [];
        for (let i = 2 + vertexCount; i < 2 + vertexCount + faceCount; i++) {
            const indices = lines[i].split(/\s+/).map(Number);
            const vertexCountInFace = indices[0];
            if (vertexCountInFace !== 3) {
                console.warn(`Non-triangular face detected at line ${i}`);
                continue;
            }
            for (let j = 1; j <= 3; j++) {
                const vertex = vertices[indices[j]];
                positions.push(vertex.x, vertex.y, vertex.z);
            }
            const v0 = vertices[indices[1]];
            const v1 = vertices[indices[2]];
            const v2 = vertices[indices[3]];
            const normal = new THREE.Vector3()
                .subVectors(v1, v0)
                .cross(new THREE.Vector3().subVectors(v2, v0))
                .normalize();
            for (let j = 0; j < 3; j++) {
                normals.push(normal.x, normal.y, normal.z);
            }
            edgePositions.push(
                v0.x, v0.y, v0.z, // Start of edge 1
                v1.x, v1.y, v1.z, // End of edge 1
                v1.x, v1.y, v1.z, // Start of edge 2
                v2.x, v2.y, v2.z, // End of edge 2
                v2.x, v2.y, v2.z, // Start of edge 3
                v0.x, v0.y, v0.z  // End of edge 3
            );
        }

        geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
        geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));

        const material = new THREE.MeshPhongMaterial({
            color: 0xcccccc, // Light gray color
            side: THREE.DoubleSide,
            flatShading: true
        });
        const mesh = new THREE.Mesh(geometry, material);
        this.scene.add(mesh);

        const edgesGeometry = new THREE.BufferGeometry();
        edgesGeometry.setAttribute('position', new THREE.Float32BufferAttribute(edgePositions, 3));
        const edgesMaterial = new THREE.LineBasicMaterial({ color: 0x000000 });
        const edges = new THREE.LineSegments(edgesGeometry, edgesMaterial);

        const group = new THREE.Group();
        group.add(mesh);
        if (this.displayEdges) {
            group.add(edges);
        }
        this.scene.add(group);

        geometry.computeBoundingBox();
        const box = geometry.boundingBox;
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);
        const scale = 4 / maxDim;

        group.position.sub(center.multiplyScalar(scale));
        group.scale.set(scale, scale, scale);

        this.camera.position.set(0, 0, maxDim * 1.5);
        this.controls.target.copy(group.position);
        this.controls.update();
    }

    animate() {
        requestAnimationFrame(() => this.animate());
        this.controls.update();
        this.renderer.render(this.scene, this.camera);
    }

    dispose() {
        window.removeEventListener('resize', () => this.onWindowResize());

        this.scene.traverse(object => {
            if (object.isMesh || object.isLineSegments) {
                object.geometry.dispose();
                if (Array.isArray(object.material)) {
                    object.material.forEach(mat => mat.dispose());
                } else {
                    object.material.dispose();
                }
            }
        });

        this.renderer.dispose();
        this.renderer.forceContextLoss();
        this.container.removeChild(this.renderer.domElement);
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        this.controls = null;
    }
}