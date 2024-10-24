import "/static/page/component/MenuItem.js";

class TopMenu extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<div class="Row">
                    <menu-item class="Selected">
                        Daily Report
                    </menu-item>
                    <menu-item>
                        Tools
                    </menu-item>
                </div>`;
    }
    getCss() {

        const style = ``;

        return style;
    }

    connectedCallback() {
        this.setAttribute("index", 0);

        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;


        this.appendChild(template.content.cloneNode(true));
        let menuItems = this.querySelectorAll("menu-item");

        menuItems.forEach((self) => {
            self.addEventListener("click", (e) => {
                if (!self.classList.contains("Selected")) {
                    self.classList.add("Selected");
                }
                menuItems.forEach((other) => {
                    if (other != self) {
                        other.classList.remove("Selected");
                    }
                })

                let index = Array.prototype.indexOf.call(menuItems, self);
                this.setAttribute("index", index);
            })
        })

    }
}

window.customElements.define("top-menu", TopMenu);
