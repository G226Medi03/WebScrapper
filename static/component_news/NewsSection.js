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
        return `<div class="WFC" style="width : ${width}">
                    <div class="MAC W100 FS_16 PTB_5" style="background-color: #3F63BF; color: white;">
                        ${section}
                    </div>
                    <div style="height : ${height}; overflow-y : scroll;">
                        ${innerHtml}
                    </div>
                </div>`;
    }
    getStyle() {
        const style = document.createElement("style");
        style.textContent = `

        `;

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


export async function Define() {
    window.customElements.define("news-section", NewsSection);
}