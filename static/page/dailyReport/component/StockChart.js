import "/static/component/NaverFinance/Logo.js";
import "/static/component/InvestingCom/Logo.js";
import "/static/component/TraidingView/Logo.js";

class StockChart extends HTMLElement {
    constructor() {
        super();
    }

    async drawStockChart(canvas) {
        const stock = this.getAttribute("stock");
        if (!stock) return;

        const utf8Stock = encodeURIComponent(stock);

        const response = await fetch(`/api/stock/price?stock=${utf8Stock}`);
        const stockData = await response.json(); // Use .json() directly instead of .text() and JSON.parse()

        const data = [{
            type: 'candlestick',
            xaxis: 'x',
            yaxis: 'y',
            decreasing: { line: { color: 'blue' }, fillcolor: "blue" },
            increasing: { line: { color: 'red' }, fillcolor: "red" },
            line: { width: 1.5 },
            x: stockData.map(d => `${d.date.slice(0, 4)}-${d.date.slice(4, 6)}-${d.date.slice(6)}`),
            open: stockData.map(d => d.open),
            close: stockData.map(d => d.close),
            high: stockData.map(d => d.high),
            low: stockData.map(d => d.low)
        }];

        const layout = {
            xaxis: { rangeslider: { visible: false }, type: 'category', showticklabels: false },
            yaxis: { type: "log" },
            margin: { l: 0, r: 0, t: 40, b: 40 }
        };

        const config = { displayModeBar: false };

        Plotly.newPlot(canvas, data, layout, config);
    }

    async getStockLinks(stock) {
        const encodedStock = encodeURIComponent(stock);
        const response = await fetch(`/api/stock/link?stock=${encodedStock}`);
        return await response.json();
    }

    async generateHtml() {
        const stock = this.getAttribute("stock") || "NULL";
        const links = await this.getStockLinks(stock);

        const investingLink = links.investing || "#";
        const naverfinanceLink = links.naverfinance || "#";
        const traidingviewLink = links.traidingview || "#";

        const canvasWidth = this.getAttribute("canvasWidth") || "340px";
        const canvasHeight = this.getAttribute("canvasHeight") || "250px";
        const linkWidth = this.getAttribute("linkWidth") || "100px";
        const linkHeight = this.getAttribute("linkHeight") || "";

        return `
            <div>
                <span>${stock}</span>
                <div class="Row">
                    <div>
                        <div class="StockCanvas" style="height: ${canvasHeight}; width: ${canvasWidth};"></div>
                    </div>
                    <div class="Col mac sac">
                        <investing-logo 
                            width="${linkWidth}" 
                            height="${linkHeight}" 
                            onclick="window.open('${investingLink}')"></investing-logo>
                        <naverfinance-logo 
                            width="${linkWidth}" 
                            height="${linkHeight}" 
                            onclick="window.open('${naverfinanceLink}')"></naverfinance-logo>
                        <traidingview-logo 
                            width="${linkWidth}" 
                            height="${linkHeight}" 
                            onclick="window.open('${traidingviewLink}')"></traidingview-logo>
                    </div>
                </div>
            </div>`;
    }

    getCss() {
        return ``;
    }

    initializeStockChart() {
        const canvas = this.querySelector(".StockCanvas");
        if (canvas) {
            this.drawStockChart(canvas);
        }
    }

    async connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
            <style>${this.getCss()}</style>
            ${await this.generateHtml()}
        `;

        this.appendChild(template.content.cloneNode(true));
        this.initializeStockChart();
    }
}

window.customElements.define("stock-chart", StockChart);

