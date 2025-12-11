(function () {
    function ready(fn) {
        if (document.readyState !== 'loading') return fn();
        document.addEventListener('DOMContentLoaded', fn);
    }

    ready(function () {
        const container = document.getElementById("dividas");

        gasto_por_categoria.forEach(categoria =>{
            let nome = categoria.grupo.replace(' ', '_')+"_chart";
            console.log(nome);

            const novaDiv = document.createElement("div");
            novaDiv.id = nome;
            novaDiv.className = "chart";
            novaDiv.style = "width:24%";
            novaDiv.setAttribute("aria-label", categoria.grupo);
            container.insertAdjacentElement("afterend", novaDiv);

            if (!window.am4core || !window.am4charts) return;

            am4core.useTheme(am4themes_animated);

            var chart = am4core.create(nome, am4charts.PieChart);
            chart.innerRadius = am4core.percent(45);
            chart.responsive.enabled = true;

            var title = chart.titles.create();
            title.text = categoria.grupo + " D30";
            title.fontSize = 16;
            title.fill = am4core.color("#e1cdcbff");
            title.marginBottom = -5;

            chart.data = categoria.valores;

            var pieSeries = chart.series.push(new am4charts.PieSeries());
            pieSeries.dataFields.value = "valor";
            pieSeries.dataFields.category = "nome";
            pieSeries.slices.template.stroke = am4core.color("#0b1220");
            pieSeries.slices.template.strokeOpacity = 0.6;
            pieSeries.labels.template.disabled = true;
            pieSeries.ticks.template.disabled = true;

            chart.legend = new am4charts.Legend();
            chart.legend.position = "right";
            chart.legend.labels.template.fill = am4core.color("#cbd5e1");
            chart.legend.valueLabels.template.fill = am4core.color("#cbd5e1");
            chart.legend.labels.template.text = "{nome}: R$ {valor}";
            chart.legend.labels.template.fontSize = 8;
            chart.legend.valueLabels.template.fontSize = 8;
        });
    });
})();
