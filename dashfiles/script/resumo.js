// formnatar em moeda brasileira
function formatarMoeda(valor) {
  return valor.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' });
}

document.getElementById("media-entradas").innerText = formatarMoeda(resumo.media_entradas);
document.getElementById("media-saidas").innerText = formatarMoeda(resumo.media_saidas);
document.getElementById("media-custo").innerText = formatarMoeda(resumo.media_custo_vida);
document.getElementById("atual-entradas").innerText = formatarMoeda(resumo.atual_entradas);
document.getElementById("atual-saidas").innerText = formatarMoeda(resumo.atual_saidas);