// chart-pie.js
// cria gráfico de pizza (distribuição despesas) usando amCharts 4
(function () {
  function ready(fn) {
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function () {
    if (!window.am4core || !window.am4charts) return;

    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("gastoPorConta", am4charts.PieChart);
    chart.innerRadius = am4core.percent(45);
    chart.responsive.enabled = true;

    var title = chart.titles.create();
    title.text = "Gastos por Conta D30";
    title.fontSize = 16;
    title.fill = am4core.color("#cbd5e1");
    title.marginBottom = -5;

    chart.data = gasto_por_conta;

    var pieSeries = chart.series.push(new am4charts.PieSeries());
    pieSeries.dataFields.value = "valor";
    pieSeries.dataFields.category = "conta";
    pieSeries.slices.template.stroke = am4core.color("#0b1220");
    pieSeries.slices.template.strokeOpacity = 0.6;
    pieSeries.labels.template.disabled = true;
    pieSeries.ticks.template.disabled = true;

    chart.legend = new am4charts.Legend();
    chart.legend.position = "right";
    chart.legend.labels.template.fill = am4core.color("#cbd5e1");
    chart.legend.valueLabels.template.fill = am4core.color("#cbd5e1");
    chart.legend.labels.template.text = "{conta}: R$ {valor}";
    chart.legend.labels.template.fontSize = 8;
    chart.legend.valueLabels.template.fontSize = 8;

  });
})();
