class NewsCard extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        let link = this.getAttribute("link");
        let title = this.getAttribute("title");
        let imgSrc = this.getAttribute("imgSrc");
        return `<div class="NewsCard Row CP M_10" OnClick="window.open('${link}', '_blank', 'noopener,noreferrer');">
                    <image src="${imgSrc}" style="width: 100px; height : 68px;"></image>
                    <div class="ML_5" style="width: 150px;">${title}</div>
                </div>`;
    }
    getCss() {
        const style = `
            .NewsCard:hover {
                background-color: lightgray;
            }`;

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
    window.customElements.define("news-card", NewsCard);
}