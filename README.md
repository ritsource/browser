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