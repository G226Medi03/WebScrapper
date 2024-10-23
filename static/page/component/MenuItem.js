
class MenuItem extends HTMLElement {
    constructor() {
        super();
    }



    getHtml() {
        let html = this.innerHTML;
        this.innerHTML = "";
        return `
            <div class="menuItem mac sac NoEnter">
                ${html}
            </div>`;
    }


    getCss() {
        let backgroundColor = this.getAttribute("backgroundColor") || "#131722";
        const style = `
            menu-item>.menuItem {
                font-size: 32px;
                border-radius: 5px;
                margin: 10px;
                padding-bottom: 10px;
                padding-top: 5px;
                padding-right: 20px;
                padding-left: 20px;
            }

            menu-item>.menuItem:hover {
                background-color: #EEEEEE;
            }

            menu-item.Selected>.menuItem {
                border-bottom-right-radius: 0px;
                border-bottom-left-radius: 0px;
            }

            menu-item.Selected>.menuItem::after {
                background-color: blue;
                height: 1px;
                width: calc(100% + 40px);
                position: relative;
                top: 10px;
                content: "";
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

window.customElements.define("menu-item", MenuItem);
