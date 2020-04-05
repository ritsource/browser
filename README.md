# General Flow

#### Parse HTML and Construct DOM Tree
Constructing `DOM` tree from HTML

#### Parse CSS
All the external CSS files, and `<style/>` elements.

#### Construct Render Tree (HTML + Style)
The render tree contains rectangles with visual attributes like color and dimensions. The rectangles are in the right order to be displayed on the screen.

#### Layout of Render Tree
Layouting means giving each `Node` the exact coordinates where it should appear on the screen.

#### Paint Render Tree
Painting each node using the UI backend layer

![A diagram](https://www.html5rocks.com/en/tutorials/internals/howbrowserswork/webkitflow.png)

by [howbrowserswork](https://www.html5rocks.com/en/tutorials/internals/howbrowserswork/#Resources)


# Tree Structure

## Document Object Model (DOM)

![A diagram](https://developers.google.com/web/fundamentals/performance/critical-rendering-path/images/dom-tree.png)

## CSS Object Model (CSSOM)

![A diagram](https://developers.google.com/web/fundamentals/performance/critical-rendering-path/images/cssom-tree.png)

source [developers.google.com](https://developers.google.com/web/fundamentals/performance/critical-rendering-path/constructing-the-object-model)

# StyleSheet Structure
```css
span {
    d1; d2; d3;
}
div > p > span {
    d3; d4;
}
p > span {
    d5; d6;
}
h1 -> span {
    d7; d8;
}
nav span {
    d9; d10;
}
```

> RESULT STYLESHEET

```javascript
const StyleSheet = [
    { // Span
        declerations: [ d1, d2, d3 ],
        direct_parents: [
            { // P
                declerations: [ d5, d6 ],
                parents: [
                    { // Div
                        declerations: [ d3, d4 ],
                        parents: []
                    }
                ]
            },
            { // H1
                declerations: [ d7, d8 ],
                parents: []
            }
        ],
        parents: [
            { // Nav
                declerations: [ d9, d10 ],
                parents: []
            }
        ]
    }
];
```

