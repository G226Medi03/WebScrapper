import _1 from "/static/component/WeatherGoKr/WeatherCard.js";

class WeatherContainer extends HTMLElement {
    constructor() {
        super();
    }

    async initWeatherCards() {

        fetch("./weathers").then(resp => resp.text())
            .then(resp => JSON.parse(resp))
            .then((resp) => {

                let htmlElem = this.querySelector(".WeatherCardContainer");

                let sHtml = "";
                resp.forEach((obj) => {
                    sHtml += `<weather-card 
                                        date="${obj["date"]}"
                                        day="${obj["day"]}" 
                                        description="${obj["description"]}"
                                        weather="${obj["weather"]}" 
                                        lowTemp="${obj["lowTemp"]}"
                                        highTemp="${obj["highTemp"]}"
                                        rainPercent="${obj["rainPercent"]}">
                                </weather-card>`;
                });
                htmlElem.innerHTML = sHtml;
            });
    }


    async updateWeatherCards() {
        //style 조정
        document.addEventListener("DOMContentLoaded", (e) => {
            this.initWeatherCards();

            let cards = [];
            this.querySelectorAll("weather-card").forEach((webComponent) => {
                let card = webComponent.querySelector(".WeatherCard");
                cards.push(card);
            });
            cards.forEach((card) => {
                card.addEventListener("click", (e) => {
                    cards.forEach((_card) => {
                        if (_card != card) {
                            _card.classList.remove("Selected");
                        }
                    });
                });
            });
        });

    }


    getHtml() {
        return `<span class="MTB_10 FS_30">
                    Weather
                </span>
                <div class="Row WFC WeatherCardContainer OX_S W_100"">
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
        this.updateWeatherCards();
    }



}

export default async function Define() {
    window.customElements.define("weather-container", WeatherContainer);
}



Define();