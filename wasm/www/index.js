import * as wasm from "water-levels-wasm";
import * as d3 from "d3";

document.querySelector("#commit").innerText = wasm.commit();

let hours = 4;

let level_el = document.querySelector("#level");
level_el.addEventListener("input", () => {
    hours_el.value = level_el.value;
    hours = level_el.value;
    calculate();
});

let hours_el = document.querySelector("#hours");
hours_el.addEventListener("input", () => {
    if (hours_el.value < 0) {
        hours_el.value = 0;
    }
    level_el.value = hours_el.value;
    hours = hours_el.value;
    calculate();
})

hours_el.value = hours;
level_el.value = hours;

let add_el = document.querySelector(".add");
let graph_el = document.querySelector("#graph");

function calculate() {
    const WIDTH = 50;
    let elevations = [];
    document.querySelectorAll(".elevation").forEach((el) => elevations.push(el.value));
    let res = wasm.calculate(elevations, hours);
    graph_el.innerHTML = "";
    let results = document.querySelectorAll(".result");
    results.forEach(el => el.innerHTML = "");
    if (res.err) {
        let pre = document.createElement("pre");
        pre.innerText = "Error: " + res.err;
        graph_el.appendChild(pre);
    } else {
        res = res.res;
        let height = Math.max(...res);
        let width = res.length * WIDTH;
        let svg = d3.select("#graph").append("svg").attr("width", width).attr("height", height);
        for (let i = 0; i < elevations.length; i++) {
            svg.append("rect")
                .attr("x", i * WIDTH)
                .attr("y", height - elevations[i])
                .attr("width", WIDTH)
                .attr("height", elevations[i])
                .attr("fill", "#15792b");
            if (res[i] > elevations[i]) {
                svg.append("rect")
                    .attr("x", i * WIDTH)
                    .attr("y", height - res[i])
                    .attr("width", WIDTH)
                    .attr("height", res[i] - elevations[i])
                    .attr("fill", "#116fc2");
            }
            results[i].innerText = res[i];
        }
    }
}

function remove(el) {
    let li = el.target.parentElement;
    li.parentElement.removeChild(li);
    calculate();
}

function update(ev) {
    if (ev.target.value < 0) {
        ev.target.value = 0;
    }
    calculate();
}

for (let button of document.querySelectorAll(".remove")) {
    button.addEventListener("click", remove);
}
document.querySelectorAll(".elevation").forEach((el) => el.addEventListener("input", update));

function add_elevation(val) {
    let li = add_el.parentElement;
    let new_ = document.createElement("li");
    new_.innerHTML = `
        <input class="elevation" type="number" value="${val}">
        <button class="remove">Remove</button>
        Result: <span class="result"></span>
    `;
    new_.querySelector("button").addEventListener("click", remove);
    new_.querySelector("input").addEventListener("input", update);
    li.parentElement.insertBefore(new_, li);
}

add_el.addEventListener("click", () => {
    add_elevation(0);
    calculate();
})

add_elevation(10);
add_elevation(30);
add_elevation(50);
add_elevation(70);
add_elevation(90);
add_elevation(0);
add_elevation(70);
add_elevation(50);
add_elevation(30);
add_elevation(10);
add_elevation(0);
add_elevation(44);
add_elevation(0);
add_elevation(0);

calculate();
