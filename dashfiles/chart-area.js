// chart-area.js
(function(){
  function ready(fn){
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function(){
    if (!window.am4core || !window.am4charts) return;
    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("chartArea", am4charts.XYChart);
    chart.responsive.enabled = true;
    chart.paddingRight = 20;

    // dados fixos: acúmulo de investimentos e poupança
    chart.data = [
      { month: "Jan", investimento: 200, poupanca: 100 },
      { month: "Fev", investimento: 300, poupanca: 120 },
      { month: "Mar", investimento: 250, poupanca: 140 },
      { month: "Abr", investimento: 400, poupanca: 160 },
      { month: "Mai", investimento: 420, poupanca: 200 },
      { month: "Jun", investimento: 520, poupanca: 260 },
      { month: "Jul", investimento: 600, poupanca: 300 }
    ];

    var categoryAxis = chart.xAxes.push(new am4charts.CategoryAxis());
    categoryAxis.dataFields.category = "month";
    categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    var valueAxis = chart.yAxes.push(new am4charts.ValueAxis());
    valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    function createArea(field, name, color){
      var series = chart.series.push(new am4charts.LineSeries());
      series.dataFields.valueY = field;
      series.dataFields.categoryX = "month";
      series.name = name;
      series.stroke = am4core.color(color);
      series.fill = am4core.color(color);
      series.fillOpacity = 0.25;
      series.strokeWidth = 2.2;
      series.tensionX = 0.8;
      series.tooltipText = name + ": R$ {valueY.formatNumber('#,###.00')}";
      series.sequencedInterpolation = true;
      return series;
    }

    createArea("investimento", "Investimentos", "#06b6d4");
    createArea("poupanca", "Poupança", "#10b981");

    chart.legend = new am4charts.Legend();
    chart.legend.labels.template.fill = am4core.color("#cbd5e1");
    chart.cursor = new am4charts.XYCursor();
    chart.exporting.menu = new am4core.ExportMenu();
  });
})();
