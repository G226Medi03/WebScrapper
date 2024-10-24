import "/static/page/AutoTools/component/YoutubeAudio.js"

class AutoTools extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<div>
                   <youtube-audio></youtube-audio>
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

window.customElements.define("auto-tools", AutoTools);
