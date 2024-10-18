export default function updateStockChart(selector, stock) {
    const utf8Arr = encodeURIComponent(stock);

    fetch(`/stock?stock=${utf8Arr}`).then(resp => { return resp.text(); }).then(data => {
        let stockData = JSON.parse(data)

        var data = [{
            type: 'candlestick',
            xaxis: 'x',
            yaxis: 'y',
            decreasing: { line: { color: 'blue' } },
            increasing: { line: { color: 'red' } },


            x: stockData.map((d) => {

                let ret = `${d["date"].slice(0, 4)}-${d["date"].slice(4, 6)}-${d["date"].slice(6)}`
                return ret;
            }),
            open: stockData.map(d => d.open),
            close: stockData.map(d => d.close),
            high: stockData.map(d => d.high),
            low: stockData.map(d => d.low)
        }];

        var layout = {
            title: stock,
            xaxis: {
                rangeslider: { visible: false },
                type: 'category',
                showticklabels: false,
            },
        };

        var config = {
            displayModeBar: false // Hide the modebar
        };
        Plotly.newPlot(document.querySelector(selector), data, layout, config);
    })
}