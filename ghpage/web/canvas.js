const canvas = document.getElementById('hero-canvas');
const ctx = canvas.getContext('2d');

let width, height;

function resize() {
    width = window.innerWidth;
    height = window.innerHeight;
    canvas.width = width;
    canvas.height = height;
}
window.addEventListener('resize', resize);
resize();

// 3D Geometry (Icosahedron-ish vertices)
const t = (1 + Math.sqrt(5)) / 2;
const vertices = [
    [-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0],
    [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t],
    [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]
];

const edges = [
    [0, 11], [0, 5], [0, 1], [0, 7], [0, 10],
    [1, 5], [1, 9], [1, 8], [1, 7],
    [2, 11], [2, 10], [2, 6], [2, 3], [2, 4],
    [3, 4], [3, 9], [3, 8], [3, 6],
    [4, 5], [4, 9], [4, 11],
    [5, 11], [5, 9], // Duplicates possible but okay for wireframe
    [6, 7], [6, 10], [6, 8],
    [7, 8], [7, 10],
    [8, 9],
    [10, 11]
];

let angleX = 0;
let angleY = 0;

function rotate(v, ax, ay) {
    let x = v[0], y = v[1], z = v[2];

    // Rotate Y
    let x1 = x * Math.cos(ay) - z * Math.sin(ay);
    let z1 = x * Math.sin(ay) + z * Math.cos(ay);

    // Rotate X
    let y1 = y * Math.cos(ax) - z1 * Math.sin(ax);
    let z2 = y * Math.sin(ax) + z1 * Math.cos(ax);

    return [x1, y1, z2];
}

function draw() {
    ctx.clearRect(0, 0, width, height);

    const scale = Math.min(width, height) / 8;
    const cx = width / 2;
    const cy = height / 2;

    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    ctx.lineWidth = 1;
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';

    angleY += 0.003;
    angleX += 0.001;

    ctx.beginPath();

    // Project vertices
    const projected = vertices.map(v => {
        const r = rotate(v, angleX, angleY);
        return [
            cx + r[0] * scale,
            cy + r[1] * scale
        ];
    });

    // Draw edges
    edges.forEach(edge => {
        const v1 = projected[edge[0]];
        const v2 = projected[edge[1]];

        ctx.moveTo(v1[0], v1[1]);
        ctx.lineTo(v2[0], v2[1]);
    });

    ctx.stroke();

    requestAnimationFrame(draw);
}

draw();
