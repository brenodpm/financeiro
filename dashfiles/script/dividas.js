// chart-column.js
(function(){
  function ready(fn){
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function(){
    if (!window.am4core || !window.am4charts) return;
    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("dividas", am4charts.XYChart);
    chart.responsive.enabled = true;
    chart.paddingRight = 20;

    // dados fixos por categoria mensal: excesso e aberto
    chart.data = dividas;

    var categoryAxis = chart.xAxes.push(new am4charts.CategoryAxis());
    categoryAxis.dataFields.category = "mes";
    categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    var valueAxis = chart.yAxes.push(new am4charts.ValueAxis());
    valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    function createColumn(field, name, color){
      var series = chart.series.push(new am4charts.ColumnSeries());
      series.dataFields.valueY = field;
      series.dataFields.categoryX = "mes";
      series.name = name;
      series.columns.template.fill = am4core.color(color);
      series.columns.template.stroke = am4core.color(color);
      series.tooltipText = name + ": R$ {valueY.formatNumber('#,###.00')}";
      series.stacked = true;
      return series;
    }

    createColumn("aberto", "Dívidas em aberto", "#eb9500ff");
    createColumn("excesso", "Dívidas além do limite", "#ce0a0aff");
    createColumn("pago", "Dívidas pagas", "#ffffffff");

    chart.legend = new am4charts.Legend();
    chart.legend.labels.template.fill = am4core.color("#cbd5e1");
    chart.legend.labels.template.fontSize = 10;
    chart.legend.valueLabels.template.fontSize = 10;
    chart.cursor = new am4charts.XYCursor();
    chart.exporting.menu = new am4core.ExportMenu();
  });
})();
