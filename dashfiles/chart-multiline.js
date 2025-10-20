// chart-multiline.js
(function(){
  function ready(fn){
    if (document.readyState !== 'loading') return fn();
    document.addEventListener('DOMContentLoaded', fn);
  }

  ready(function(){
    if (!window.am4core || !window.am4charts) return;
    am4core.useTheme(am4themes_animated);

    var chart = am4core.create("chartMultiLine", am4charts.XYChart);
    chart.paddingRight = 20;
    chart.responsive.enabled = true;

    // dados fixos: saldo, receita e despesa por mês
    chart.data = [
      { month: "Jan", saldo: 4200, receita: 5200, despesa: 1000 },
      { month: "Fev", saldo: 4800, receita: 5400, despesa: 600 },
      { month: "Mar", saldo: 5100, receita: 5800, despesa: 700 },
      { month: "Abr", saldo: 4600, receita: 5000, despesa: 1200 },
      { month: "Mai", saldo: 5200, receita: 6100, despesa: 900 },
      { month: "Jun", saldo: 5450, receita: 6300, despesa: 850 },
      { month: "Jul", saldo: 6000, receita: 7000, despesa: 1000 },
      { month: "Ago", saldo: 6250, receita: 7200, despesa: 950 },
      { month: "Set", saldo: 6100, receita: 6900, despesa: 800 },
      { month: "Out", saldo: 7300, receita: 8100, despesa: 800 },
      { month: "Nov", saldo: 7950, receita: 8600, despesa: 650 },
      { month: "Dez", saldo: 8450, receita: 9200, despesa: 750 }
    ];

    var categoryAxis = chart.xAxes.push(new am4charts.CategoryAxis());
    categoryAxis.dataFields.category = "month";
    categoryAxis.renderer.minGridDistance = 20;
    categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    var valueAxis = chart.yAxes.push(new am4charts.ValueAxis());
    valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");

    // função para criar séries
    function createSeries(field, name, color, strokeWidth, fillOpacity){
      var series = chart.series.push(new am4charts.LineSeries());
      series.dataFields.valueY = field;
      series.dataFields.categoryX = "month";
      series.name = name;
      series.stroke = am4core.color(color);
      series.strokeWidth = strokeWidth || 2.5;
      series.tensionX = 0.8;
      series.tooltipText = name + ": R$ {valueY.formatNumber('#,###.00')}";
      if (fillOpacity){
        series.fillOpacity = fillOpacity;
        series.fill = am4core.color(color);
      }
      var bullet = series.bullets.push(new am4charts.CircleBullet());
      bullet.circle.radius = 3.5;
      bullet.circle.fill = am4core.color("#0b1220");
      return series;
    }

    createSeries("saldo", "Saldo", "#60a5fa", 3, 0.12);
    createSeries("receita", "Receita", "#10b981", 2.5, 0.06);
    createSeries("despesa", "Despesa", "#ef4444", 2.5, 0);

    chart.legend = new am4charts.Legend();
    chart.legend.labels.template.fill = am4core.color("#cbd5e1");
    chart.cursor = new am4charts.XYCursor();
    chart.cursor.xAxis = categoryAxis;
    chart.exporting.menu = new am4core.ExportMenu();
  });
})();
