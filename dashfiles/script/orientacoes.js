// prioridade → cor da borda esquerda
function orientacaoCor(prioridade) {
  if (prioridade <= 20) return "#ef4444"; // vermelho — urgente
  if (prioridade <= 40) return "#f97316"; // laranja — atenção
  return "#60a5fa";                       // azul — informativo
}

(function () {
  var lista = document.getElementById("orientacoes-lista");
  if (!lista || !orientacoes || !orientacoes.length) return;

  orientacoes.forEach(function (o) {
    var item = document.createElement("div");
    item.className = "orientacao-item";
    item.style.setProperty("--orientacao-cor", orientacaoCor(o.prioridade));

    item.innerHTML =
      '<span class="orientacao-icone">' + o.icone + '</span>' +
      '<span class="orientacao-texto">' + o.texto + '</span>';

    lista.appendChild(item);
  });
})();
