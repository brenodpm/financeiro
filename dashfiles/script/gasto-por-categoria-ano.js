(function () {
    function ready(fn) {
        if (document.readyState !== 'loading') return fn();
        document.addEventListener('DOMContentLoaded', fn);
    }

    ready(function () {
        if (!window.am4core || !window.am4charts) return;

        const container = document.getElementById("lb-gastos-ano");

        gasto_por_categoria_ano.forEach(function (grafico) {
            const id = grafico.grupo.replace(/\s/g, '_') + "_ano_chart";

            const div = document.createElement("div");
            div.id = id;
            div.className = "chart";
            div.style = "width:48%";
            div.setAttribute("aria-label", grafico.grupo);
            container.insertAdjacentElement("afterend", div);

            am4core.useTheme(am4themes_animated);

            var chart = am4core.create(id, am4charts.XYChart);
            chart.responsive.enabled = true;

            var title = chart.titles.create();
            title.text = grafico.grupo;
            title.fontSize = 14;
            title.fill = am4core.color("#e1cdcbff");
            title.marginBottom = 5;

            // meses são todas as chaves exceto "categoria"
            var meses = Object.keys(grafico.valores[0]).filter(k => k !== "categoria").sort();

            chart.data = grafico.valores;

            var categoryAxis = chart.yAxes.push(new am4charts.CategoryAxis());
            categoryAxis.dataFields.category = "categoria";
            categoryAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");
            categoryAxis.renderer.labels.template.fontSize = 10;
            categoryAxis.renderer.minGridDistance = 10;

            var valueAxis = chart.xAxes.push(new am4charts.ValueAxis());
            valueAxis.renderer.labels.template.fill = am4core.color("#cbd5e1");
            valueAxis.renderer.labels.template.fontSize = 10;

            meses.forEach(function (mes) {
                var series = chart.series.push(new am4charts.ColumnSeries());
                series.dataFields.valueX = mes;
                series.dataFields.categoryY = "categoria";
                series.name = mes;
                series.stacked = true;
                series.tooltipText = mes + ": R$ {valueX.formatNumber('#,###.00')}";
                series.tooltip.label.fontSize = 10;
                series.columns.template.strokeOpacity = 0;
            });

            chart.legend = new am4charts.Legend();
            chart.legend.position = "bottom";
            chart.legend.labels.template.fill = am4core.color("#cbd5e1");
            chart.legend.labels.template.fontSize = 10;
            chart.legend.valueLabels.template.disabled = true;

            chart.cursor = new am4charts.XYCursor();
        });
    });
})();
