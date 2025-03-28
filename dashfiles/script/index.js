am5.ready(function () {
  renderizarDividas();
});


function renderizarDividas() {
  // Create root element
  // https://www.amcharts.com/docs/v5/getting-started/#Root_element
  var root = am5.Root.new("dividas");

  // Set themes
  // https://www.amcharts.com/docs/v5/concepts/themes/
  root.setThemes([
    am5themes_Animated.new(root)
  ]);


  // Create chart
  // https://www.amcharts.com/docs/v5/charts/xy-chart/
  var chart = root.container.children.push(am5xy.XYChart.new(root, {
    panX: false,
    panY: false,
    wheelX: "panX",
    wheelY: "zoomX",
    paddingLeft: 0,
    layout: root.verticalLayout
  }));

  // Add scrollbar
  // https://www.amcharts.com/docs/v5/charts/xy-chart/scrollbars/
  chart.set("scrollbarX", am5.Scrollbar.new(root, {
    orientation: "horizontal"
  }));




  // Create axes
  // https://www.amcharts.com/docs/v5/charts/xy-chart/axes/
  var xRenderer = am5xy.AxisRendererX.new(root, {
    minorGridEnabled: true
  });
  var xAxis = chart.xAxes.push(am5xy.CategoryAxis.new(root, {
    categoryField: "mes",
    renderer: xRenderer,
    tooltip: am5.Tooltip.new(root, {})
  }));

  xRenderer.grid.template.setAll({
    location: 1
  })

  xAxis.data.setAll(dividas);

  var yAxis = chart.yAxes.push(am5xy.ValueAxis.new(root, {
    min: 0,
    renderer: am5xy.AxisRendererY.new(root, {
      strokeOpacity: 0.1
    })
  }));


  // Add legend
  // https://www.amcharts.com/docs/v5/charts/xy-chart/legend-xy-series/
  var legend = chart.children.push(am5.Legend.new(root, {
    centerX: am5.p50,
    x: am5.p50
  }));


  // Add series
  // https://www.amcharts.com/docs/v5/charts/xy-chart/series/
  function makeSeries(name, fieldName) {
    var series = chart.series.push(am5xy.ColumnSeries.new(root, {
      name: name,
      stacked: true,
      xAxis: xAxis,
      yAxis: yAxis,
      valueYField: fieldName,
      categoryXField: "mes"
    }));

    series.columns.template.setAll({
      tooltipText: "{name}, {categoryX}: {valueY}",
      tooltipY: am5.percent(10)
    });
    series.data.setAll(dividas);

    // Make stuff animate on load
    // https://www.amcharts.com/docs/v5/concepts/animations/
    series.appear();

    series.bullets.push(function () {
      return am5.Bullet.new(root, {
        sprite: am5.Label.new(root, {
          text: "{valueY}",
          fill: root.interfaceColors.get("alternativeText"),
          centerY: am5.p50,
          centerX: am5.p50,
          populateText: true
        })
      });
    });

    legend.data.push(series);
  }

  makeSeries("Pago", "pago");
  makeSeries("Aberto", "aberto");
  makeSeries("Excesso", "excesso");


  chart.appear(1000, 100);

}; // end am5.ready()