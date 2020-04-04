const fs = require("fs");

function main() {
    fs.readFile("./element_tags.current.txt", (err, data) => {
        let new_data = "";

        if (err) {
            console.log("err", err);
            return;
        }

        const str = data.toString();

        let word = "";
        let t1 = "";
        let t2 = "";

        for (let i = 0; i < str.length; i++) {
            const d = str[i];
            if (d === ",") {
                t1 += word;
                word = "";
            } else if (d === "\n") {
                t2 += word;
                new_data += t2 + "," + t1 + "\n";
                word = "";
                t1 = "";
                t2 = "";
            } else {
                word += d;
            }
        }

        console.log("new_data", new_data);
    });
}

main();