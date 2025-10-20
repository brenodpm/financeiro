// Dados falsos
const metricsData = [
  { label: "Receita", value: 12450 },
  { label: "Novos Usuários", value: 342 },
  { label: "Conversão", value: "4.8%" },
  { label: "Carrinhos Abandonados", value: 27 }
];

const salesLast7 = [120, 180, 140, 210, 190, 230, 260];

const transactions = Array.from({length:18}, (_,i) => {
  const statuses = ["paid","pending","failed"];
  const s = statuses[Math.floor(Math.random()*statuses.length)];
  return {
    id: `TX${1000+i}`,
    customer: ["Ana","Bruno","Camila","Diego","Eduarda","Fabio"][i%6],
    value: (Math.random()*300 + 20).toFixed(2),
    date: new Date(Date.now() - i*86400000).toLocaleDateString(),
    status: s
  };
});

// Renderiza cards de métricas
function renderMetrics(){
  const container = document.getElementById('metrics');
  container.innerHTML = '';
  metricsData.forEach(m => {
    const el = document.createElement('div');
    el.className = 'card metric';
    el.innerHTML = `<div class="value">${m.value}</div><div class="label">${m.label}</div>`;
    container.appendChild(el);
  });
}

// Desenha gráfico simples usando Canvas
function drawSalesChart(){
  const canvas = document.getElementById('salesChart');
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0,0,w,h);

  // margens
  const pad = 30;
  const max = Math.max(...salesLast7) * 1.1;
  const stepX = (w - pad*2) / (salesLast7.length -1);

  // linhas de grade
  ctx.strokeStyle = 'rgba(255,255,255,0.04)';
  ctx.lineWidth = 1;
  for(let y=0;y<=4;y++){
    const yy = pad + (h - pad*2) * (y/4);
    ctx.beginPath();
    ctx.moveTo(pad, yy);
    ctx.lineTo(w-pad, yy);
    ctx.stroke();
  }

  // curva
  ctx.beginPath();
  ctx.strokeStyle = '#60a5fa';
  ctx.lineWidth = 2.4;
  salesLast7.forEach((val,i) => {
    const x = pad + i*stepX;
    const y = pad + (1 - val/max)*(h - pad*2);
    if(i===0) ctx.moveTo(x,y); else ctx.lineTo(x,y);
    // ponto
    ctx.fillStyle = '#93c5fd';
    ctx.beginPath();
    ctx.arc(x,y,3,0,Math.PI*2);
    ctx.fill();
  });
  ctx.stroke();
}

// Renderiza tabela e filtros
function renderTable(filter='all'){
  const tbody = document.querySelector('#txTable tbody');
  tbody.innerHTML = '';
  const rows = transactions.filter(t => filter==='all' ? true : t.status===filter);
  rows.forEach(r => {
    const tr = document.createElement('tr');
    tr.innerHTML = `<td>${r.id}</td><td>${r.customer}</td><td>R$ ${r.value}</td><td>${r.date}</td><td class="status-${r.status}">${r.status}</td>`;
    tbody.appendChild(tr);
  });
}

// Eventos
document.getElementById('statusFilter').addEventListener('change', (e) => {
  renderTable(e.target.value);
});

document.getElementById('search').addEventListener('input', (e) => {
  const q = e.target.value.toLowerCase();
  const tbody = document.querySelector('#txTable tbody');
  Array.from(tbody.rows).forEach(row => {
    const text = row.innerText.toLowerCase();
    row.style.display = text.includes(q) ? '' : 'none';
  });
});

// inicialização
renderMetrics();
drawSalesChart();
renderTable();
