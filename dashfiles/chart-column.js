// chart-column.js
(function(){
  function ready(fn){
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function(){
    if (!window.am4core || !window.am4charts) return;
    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("chartColumn", am4charts.XYChart);
    chart.responsive.enabled = true;
    chart.paddingRight = 20;

    // dados fixos por categoria mensal: fixas e variaveis
    chart.data = [
      { month: "Jan", fixas: 1200, variaveis: 800 },
      { month: "Fev", fixas: 1100, variaveis: 700 },
      { month: "Mar", fixas: 1300, variaveis: 750 },
      { month: "Abr", fixas: 1400, variaveis: 900 },
      { month: "Mai", fixas: 1250, variaveis: 950 },
      { month: "Jun", fixas: 1200, variaveis: 800 }
    ];

    var categoryAxis = chart.xAxes.push(new am4charts.CategoryAxis());
    categoryAxis.dataFields.category = "month";
    categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    var valueAxis = chart.yAxes.push(new am4charts.ValueAxis());
    valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    function createColumn(field, name, color){
      var series = chart.series.push(new am4charts.ColumnSeries());
      series.dataFields.valueY = field;
      series.dataFields.categoryX = "month";
      series.name = name;
      series.columns.template.fill = am4core.color(color);
      series.columns.template.stroke = am4core.color(color);
      series.tooltipText = name + ": R$ {valueY.formatNumber('#,###.00')}";
      series.stacked = true;
      return series;
    }

    createColumn("fixas", "Despesas Fixas", "#6366f1");
    createColumn("variaveis", "Despesas Variáveis", "#f59e0b");

    chart.legend = new am4charts.Legend();
    chart.legend.labels.template.fill = am4core.color("#cbd5e1");
    chart.cursor = new am4charts.XYCursor();
    chart.exporting.menu = new am4core.ExportMenu();
  });
})();
