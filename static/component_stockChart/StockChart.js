class StockChart extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<div class="NewsCard Row CP M_10" OnClick="window.open('${link}', '_blank', 'noopener,noreferrer');">
                    <image src="${imgSrc}" style="width: 100px; height : 68px;"></image>
                    <div class="ML_5" style="width: 150px;">${title}</div>
                </div>`;
    }
    getStyle() {
        const style = document.createElement("style");
        style.textContent = ``;

        return style;
    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getStyle().textContent}
        </style>
        ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
    }
}
window.customElements.define("stock-chart", StockChart);