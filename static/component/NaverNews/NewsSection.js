class NewsSection extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        let section = this.getAttribute("section");
        let height = this.getAttribute("height") || "400px";
        let width = this.getAttribute("width") || "280px";

        let innerHtml = this.innerHTML;
        this.innerHTML = "";
        return `<div class="wfc" style="width : ${width}">
                    <div class="mac w_100 fs_16 PTB_5" style="background-color: #3F63BF; color: white;">
                        ${section}
                    </div>
                    <div style="height : ${height}; overflow-y : scroll;">
                        ${innerHtml}
                    </div>
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

window.customElements.define("news-section", NewsSection);
