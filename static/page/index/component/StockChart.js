import _1 from "/static/component/NaverFinance/Logo.js";
import _2 from "/static/component/InvestingCom/Logo.js";
import _3 from "/static/component/TraidingView/Logo.js";

class StockChart extends HTMLElement {
    constructor() {
        super();
    }
    drawStockChart(elem) {
        let stock = this.getAttribute("stock");
        if (stock == "null") {
            return;
        }

        const utf8Arr = encodeURIComponent(stock);

        fetch(`/stock/price?stock=${utf8Arr}`).then(resp => { return resp.text(); }).then(data => {
            let stockData = JSON.parse(data)

            var data = [{
                type: 'candlestick',
                xaxis: 'x',
                yaxis: 'y',
                decreasing: { line: { color: 'blue' }, fillcolor: "blue" },
                increasing: { line: { color: 'red' }, fillcolor: "red" },
                line: {
                    width: 1.5,
                },

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
                xaxis: {
                    rangeslider: { visible: false },
                    type: 'category',
                    showticklabels: false,
                },
                yaxis: {
                    type: "log",
                },
                margin: {
                    l: 0,
                    r: 0,
                    t: 40,
                    b: 40,
                }
            };

            var config = {
                displayModeBar: false // Hide the modebar
            };
            Plotly.newPlot(elem, data, layout, config);
        })
    }

    getLink(stock) {
        const encodedStock = encodeURIComponent(stock);
        let resp = fetch(`/stock/link?stock=${encodedStock}`)
            .then(resp =>
                resp.text()
            )
            .then(resp => JSON.parse(resp))
        return resp;
    }
    getHtml() {

        let stock = this.getAttribute("stock") || "NULL";

        let links = this.getLink(stock);
        let investingLink = links["investing"];
        let naverfinanceLink = links["naverfinance"];
        let traidingviewLink = links["traidingview"];
        let canvasWidth = this.getAttribute("canvasWidth") || "340px";
        let canvasHeight = this.getAttribute("canvasHeight") || "250px";
        let linkWidth = this.getAttribute("linkWidth") || "100px";
        let linkHeight = this.getAttribute("linkHeight") || "";


        return `
            <div>
                <span>
                    ${stock}
                </span>
                <div class="Row">
                    <div>
                        <div class="StockCanvas" style="height : ${canvasHeight}; width: ${canvasWidth};">
                        </div>
                    </div>
                    <div class="Col MAC SAC">
                        <investing-logo 
                                width = "${linkWidth}"
                                height = "${linkHeight}"
                                onclick="window.open('${investingLink}')"></investing-logo>
                        <naverfinance-logo
                                width = "${linkWidth}"
                                height = "${linkHeight}"
                                onclick="window.open('${naverfinanceLink}')"></naverfinance-logo>
                        <traidingview-logo
                                width = "${linkWidth}"
                                height = "${linkHeight}"
                                onclick="window.open('${traidingviewLink}')"></traidingview-logo>
                    </div>
                </div>
            </div>`;
    }
    getCss() {
        const style = ``;

        return style;
    }

    runJs() {
        let canvas = this.querySelector(".StockCanvas")
        if (canvas != null && canvas != undefined) {
            this.drawStockChart(canvas);
        }
    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;
        this.appendChild(template.content.cloneNode(true));

        this.runJs();
    }
}

export default function Define() {
    window.customElements.define("stock-chart", StockChart);
}

Define();