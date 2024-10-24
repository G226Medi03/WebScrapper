import "/static/page/dailyReport/component/container/NewsContainer.js";
import "/static/page/dailyReport/component/container/WeatherContainer.js";
import "/static/page/dailyReport/component/container/StockContainer.js";

class DailyReport extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<div>
                    <weather-container></weather-container>
                    <stock-container></stock-container>
                    <news-container></news-container>
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

window.customElements.define("daily-report", DailyReport);
