import init from "./editor.js";
import { run, load_shader, get_url, load_image } from "./editor.js";

window.addEventListener("shader-loaded", (e) => {
    const editor = document.querySelector("#code-editor");
    editor.value = e.detail;
});

document.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#update_button").addEventListener("click", () => {
        const code = document.querySelector("#code-editor").value;
        if (code) {
            load_shader(code);
        }
    });

    document.querySelector("#copy_url").addEventListener("click", async () => {
        try {
            const url = `${window.location.host}?config=${get_url()}`;
            await navigator.clipboard.writeText(url);
        } catch (err) {
            console.log(err);
        }
    });

    document.querySelector("#texture_button").addEventListener("click", (event) => {
        document.querySelector("#texture_input").click();
    });

    document.querySelector("#texture_input").addEventListener("change", (event) => {
        const file = event.target.files[0];
        document.querySelector("#texture_button").innerHTML = file.name;

        console.log(file);
        if (file) {
            const reader = new FileReader();
            reader.onload = function (e) {
                const bytes = new Uint8Array(e.target.result);
                console.log("loading texture");
                load_image(bytes, "test");
            };
            reader.readAsArrayBuffer(file);
        }
    });

    start().catch(console.error);
});

async function start() {
    await init();
    const params = new URL(window.location.toString()).searchParams;
    const config = params.get("config");
    await run(config);
}
