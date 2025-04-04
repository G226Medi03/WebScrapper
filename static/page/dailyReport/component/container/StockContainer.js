import "/static/page/dailyReport/component/StockChart.js"

class StockContainer extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `
                <div class="mtb_20 fs_32">
                    <span class="fs_32">
                        Stock Charts
                    </span>
                </div>
                <div class="Row O_F mac" style="padding-left: 25px; gap : 0px 20px;">
                    <stock-chart stock="Nasdaq"></stock-chart>
                    <stock-chart stock="TMF"/></stock-chart>
                    <stock-chart stock="BTC"></stock-chart>
                    <stock-chart stock="OILK"></stock-chart>
                    <stock-chart stock="KT&G"></stock-chart>
                    <stock-chart stock="Samsung"></stock-chart>
                </div>`;
    }
    getCss() {
        const style = ``;

        return style;
    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
    }
}

window.customElements.define("stock-container", StockContainer);