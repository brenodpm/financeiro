// chart-line.js
// cria gráfico de linha (saldo mensal) usando amCharts 4
(function(){
  // espera DOM estar pronto e amCharts carregado
  function ready(fn){
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function(){
    if (!window.am4core || !window.am4charts) return;

    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("chartLine", am4charts.XYChart);
    chart.paddingRight = 20;
    chart.responsive.enabled = true;

    chart.data = [
      { month: "Jan", saldo: 4200 },
      { month: "Fev", saldo: 4800 },
      { month: "Mar", saldo: 5100 },
      { month: "Abr", saldo: 4600 },
      { month: "Mai", saldo: 5200 },
      { month: "Jun", saldo: 5450 },
      { month: "Jul", saldo: 6000 },
      { month: "Ago", saldo: 6250 },
      { month: "Set", saldo: 6100 },
      { month: "Out", saldo: 7300 },
      { month: "Nov", saldo: 7950 },
      { month: "Dez", saldo: 8450 }
    ];

    var categoryAxis = chart.xAxes.push(new am4charts.CategoryAxis());
    categoryAxis.dataFields.category = "month";
    categoryAxis.renderer.grid.template.stroke = am4core.color("#1f2937");
    categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");
    categoryAxis.renderer.minGridDistance = 20;

    var valueAxis = chart.yAxes.push(new am4charts.ValueAxis());
    valueAxis.renderer.grid.template.stroke = am4core.color("#111827");
    valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");
    valueAxis.tooltip.disabled = true;

    var series = chart.series.push(new am4charts.LineSeries());
    series.dataFields.valueY = "saldo";
    series.dataFields.categoryX = "month";
    series.strokeWidth = 3;
    series.fillOpacity = 0.15;
    series.stroke = am4core.color("#60a5fa");
    series.tensionX = 0.8;
    series.tooltipText = "R$ {valueY}";

    var bullet = series.bullets.push(new am4charts.CircleBullet());
    bullet.circle.radius = 4;
    bullet.circle.strokeWidth = 2;
    bullet.circle.fill = am4core.color("#0b1220");

    chart.cursor = new am4charts.XYCursor();
    chart.cursor.xAxis = categoryAxis;
    chart.exporting.menu = new am4core.ExportMenu();
  });
})();
