am5.ready(function () {
  renderizarDividas();
});


function renderizarDividas() {
  var root = am5.Root.new("dividas");
  root.setThemes([
    am5themes_Animated.new(root),
    am5themes_Dark.new(root)
  ]);

  var chart = root.container.children.push(am5xy.XYChart.new(root, {
    panX: false,
    panY: false,
    wheelX: "panX",
    wheelY: "zoomX",
    paddingLeft: 0,
    layout: root.verticalLayout
  }));

  var xRenderer = am5xy.AxisRendererX.new(root, {
    minorGridEnabled: true
  });

  var xAxis = chart.xAxes.push(am5xy.CategoryAxis.new(root, {
    categoryField: "mes",
    renderer: xRenderer,
    tooltip: am5.Tooltip.new(root, {})
  }));

  xAxis.get("renderer").labels.template.setAll({
    rotation: -75,
    fontSize: 9,
    centerY: am5.p50,
    centerX: am5.p100,
  });

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

  var legend = chart.children.push(am5.Legend.new(root, {
    centerX: am5.p50,
    x: am5.p50
  }));

  legend.labels.template.setAll({
    fontSize: 8
    
});

  function makeSeries(name, fieldName, cor) {
    var series = chart.series.push(am5xy.ColumnSeries.new(root, {
      name: name,
      stacked: true,
      xAxis: xAxis,
      yAxis: yAxis,
      valueYField: fieldName,
      categoryXField: "mes",
      fill: cor,

    }));

    series.columns.template.setAll({
      tooltipText: "{categoryX}: R${valueY.formatNumber('#,##0.00')}",
      tooltipY: am5.percent(10)
    });
    series.data.setAll(dividas);


    series.appear();

    series.bullets.push(function (root, series, dataItem) {
      if (dataItem.get("valueY") !== 0) {
        return am5.Bullet.new(root, {
          sprite: am5.Label.new(root, {
            text: "{valueY}",
            fill: am5.color(0xffffff),
            centerY: am5.p50,
            centerX: am5.p50,
            populateText: true,
            fontSize: 12,
          })
        });
      }
      return null;
    });

    legend.data.push(series);
  }

  makeSeries("Pago", "pago", am5.color(0xAAAAAA));
  makeSeries("Aberto", "aberto", am5.color(0x2b6218));
  makeSeries("Excesso", "excesso", am5.color(0x710404));


  chart.appear(1000, 100);

};