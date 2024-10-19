import _5 from "/static/page/index/component/StockChart.js"
class StockContainer extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<span class="MTB_10 FS_30">
                    Stock Charts
                </span>
                <div class="Row O_F" style="padding-left: 25px;">
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

export default async function Define() {
    window.customElements.define("stock-container", StockContainer);
}



Define();