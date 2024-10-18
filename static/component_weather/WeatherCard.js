class WeatherCard extends HTMLElement {
  constructor() {
    super();
  }

  convert_weather2ImgSrc(weather) {
    let src = `src="./static/component_weather/png/${this.getAttribute("weather")}.png"`;
    return `<image class="W_50 MLR_5" ${weather == `-` ? `` : src}>
                    ${weather == `-` ? `-` : ``}
                </image>`;
  }
  getHtml(date, day, description, weather, lowTemp, highTemp, rainPercent) {
    date = this.getAttribute("date");
    day = this.getAttribute("day");
    description = this.getAttribute("description");
    weather = this.getAttribute("weather");
    lowTemp = this.getAttribute("lowTemp");
    highTemp = this.getAttribute("highTemp");
    rainPercent = this.getAttribute("rainPercent");

    let weatherImageHtml = this.convert_weather2ImgSrc(weather);
    rainPercent = rainPercent == "null" ? `-` : `${rainPercent}%`;
    highTemp = highTemp == "null" ? "-" : `${highTemp}℃`;
    lowTemp = lowTemp == "null" ? "-" : `${lowTemp}℃`;
    let fontColor;
    if (day == "토") {
      fontColor = "#0282DD";
    } else if (day == "일") {
      fontColor = "#DB0015";
    }

    return `
            <div class="WeatherCard WFC SAC MAC" id="weatherCard" style="width : 110px">
                <div class="WeatherCard_Head MAC W100" ${
                  fontColor == undefined ? "" : `style="color : ${fontColor}"`
                }>
                    <div class="SAC FS_16 H_25 FW_600 PRL_15 ">${date}일(${day})</div>
                    <div class="MAC SAC FS_14 H_20">${description}</div>
                </div>
                <div class="WeatherCard_Body">
                    
                    <div class="Row SAC H_50 MAC">
                        ${weatherImageHtml}
                    </div>
                    <div class="BB Row H_30 SAC Temperature">
                        <span class="W_50 MLR_5">${lowTemp}</span>
                        <span class="W_50 MLR_5">${highTemp}</span>
                    </div>
                    <div class="Row H_20 SAC MAC">
                        <span class="W_50 MLR_5 MAC">${rainPercent}</span>
                    </div>
                </div>
            </div>`.replaceAll("\n", "");
  }
  getStyle() {
    const style = document.createElement("style");
    style.textContent = `
            .WeatherCard_Body {
              width : 100%;
              margin-top : 5px;
              margin-bottom: 5px;
            }
            .WeatherCard {
                background-color: white;
                position: relative;

                border-right: 1px solid transparent;
                border-left: 1px solid transparent;
                transition: all 0.1s;
            }
            .WeatherCard::before {
                background-color: transparent;
                height: 2px;
                width: calc(100% + 2px);

                position: relative;
                content: "";

                transition: all 0.1s;
            }
            .WeatherCard>.WeatherCard_Head {
                background-color: #DCE1E9;

                border: 1px solid rgb(197, 207, 219);

                position: relative;
                top: -2px;

                transition: all 0.1s;
            } 
            .WeatherCard>.WeatherCard_Body>.BB {
                border-bottom: 1px solid #C9D9EF;
                transition: all 0.1s;
            }
            .WeatherCard_Body>.Temperature>span:first-child {
                color: #0083DD;
            }
            .WeatherCard_Body>.Temperature>span:last-child {
                color: #DB0015;
            }
            .Hovered, .Selected{
                border-right: 1px solid rgb(197, 207, 219);
                border-left: 1px solid rgb(197, 207, 219);

                transition: all 0.1s;
            }
            .Hovered::before, .Selected::before {
                background-color: #2CABD0;
                height: 2px;
                width: calc(100% + 2px);

                position: relative;
                content: "";

                transition: all 0.1s;
            }
            .Hovered>.WeatherCard_Head, .Selected>.WeatherCard_Head {
                background-color: transparent;

                border: 1px solid transparent;

                position: relative;
                top: -2px;

                transition: all 0.1s;
            }
            .Hovered>.WeatherCard_Body>.BB, .Selected>.WeatherCard_Body>.BB {
                border-bottom: 1px solid #E4E4E4;
                transition: all 0.1s;
            }`;

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
    let weatherCardChild = this.querySelector("div");
    weatherCardChild.addEventListener("mouseenter", (e) => {
      if (!weatherCardChild.classList.contains("Hovered")) {
        weatherCardChild.classList.add("Hovered");
      }
    });

    weatherCardChild.addEventListener("mouseleave", (e) => {
      weatherCardChild.classList.remove("Hovered");
    });
    weatherCardChild.addEventListener("click", (e) => {
      if (!weatherCardChild.classList.contains("Selected")) {
        weatherCardChild.classList.add("Selected");
      } else {
        weatherCardChild.classList.remove("Selected");
      }
    });
  }
}


export function Define() {
  window.customElements.define("weather-card", WeatherCard);

}
