import "/static/component/WeatherGoKr/WeatherCard.js";

class WeatherContainer extends HTMLElement {
    constructor() {
        super();
        this.cards = [];
    }

    drawWeatherCards() {
        fetch("./api/weathers")
            .then(resp => resp.json())  // Directly parse the JSON
            .then((resp) => {
                let weatherContainer = this.querySelector(".Row");

                resp.forEach((obj) => {
                    let weatherCard = document.createElement("weather-card");
                    weatherCard.setAttribute("date", obj["date"]);
                    weatherCard.setAttribute("day", obj["day"]);
                    weatherCard.setAttribute("description", obj["description"]);
                    weatherCard.setAttribute("weather", obj["weather"]);
                    weatherCard.setAttribute("lowTemp", obj["lowTemp"]);
                    weatherCard.setAttribute("highTemp", obj["highTemp"]);
                    weatherCard.setAttribute("rainPercent", obj["rainPercent"]);

                    this.cards.push(weatherCard);
                    weatherContainer.appendChild(weatherCard);
                });

                this.updateWeatherCards();  // Move the update call here
            });
    }

    updateWeatherCards() {
        //style adjustments
        this.cards.forEach((card) => {
            card.addEventListener("click", (_) => {

                this.cards.forEach((_card) => {
                    if (_card !== card) {
                        _card.querySelector("div").classList.remove("Selected");
                    }
                });
            });
        });
    }

    getHtml() {
        return `
                <div class="WeatherCardHead MTB_20">
                    <span class="fs_30">
                        Weather
                    </span>
                </div>
                <div class="Row wfc WeatherCardContainer OX_S w_100">
                </div>`;  // Removed the extra "
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
        this.drawWeatherCards();  // Updates are handled inside drawWeatherCards now
    }
}

window.customElements.define("weather-container", WeatherContainer);
