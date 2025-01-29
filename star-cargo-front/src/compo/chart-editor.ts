import { browser } from '$app/environment';

class PickerItem {
    value: number;
    color: string;
    length: number;
    width: number;
    height: number;

    constructor(
        value: number,
        color: string,
        length: number = 1,
        width: number = 1,
        height: number = 1,
    ) {
        this.value = value;
        this.color = color;
        this.length = length;
        this.width = width;
        this.height = height;
    }

    setDimensions(length: number, width: number, height: number) {
        this.length = length;
        this.width = width;
        this.height = height;
    }

    getDimensions(): [number, number, number] {
        return [this.length, this.width, this.height];
    }
}

class Cell {
    x: number;
    y: number;
    height: number;

    constructor(x: number, y: number, height: number) {
        this.x = x;
        this.y = y;
        this.height = height;
    }
}

class Floor {
    private cells: Cell[] = [];
    length: number = 0;
    width: number = 0;
    name: string = 'New Floor';

    constructor(
        length: number = 10,
        width: number = 10,
        name: string = 'New Floor',
    ) {
        this.length = length;
        this.width = width;
        this.name = name;
        console.log(
            'creating new floor of width',
            width,
            'and length',
            length,
            'with name',
            name,
        );
    }

    scuCount(): number {
        let res = 0;

        for (let cell of this.cells) {
            res += cell.height;
        }

        return res;
    }

    getCellAt(x: number, y: number): Cell | null {
        let ret = null;

        let i = 0;
        while (i < this.cells.length && !ret) {
            let cell = this.cells[i];
            if (cell.x === x && cell.y === y) {
                ret = cell;
            }
            i++;
        }

        return ret;
    }

    addCell(x: number, y: number, height: number = 0) {
        let cell = new Cell(x, y, height);
        this.cells.push(cell);
    }
}

class ChartEditor {
    private canvas: HTMLCanvasElement | undefined;
    private floors: Floor[] = [];
    private currentFloor: Floor;

    private CELL_SIZE = 20;
    private CELL_GAP = 5;

    private pickerItems: PickerItem[] = [
        new PickerItem(1, 'gray', 1, 1, 1),
        new PickerItem(2, 'lightblue', 2, 1, 1),
        new PickerItem(4, 'white', 2, 2, 1),
        new PickerItem(8, 'yellow', 2, 2, 2),
        new PickerItem(16, 'green', 4, 2, 2),
        new PickerItem(24, 'blue', 6, 2, 2),
        new PickerItem(32, 'brown', 8, 2, 2),
    ];

    constructor() {
        console.log('creating chart editor');

        this.currentFloor = this.addFloor();
        console.log('chart editor created');
    }

    createDefaultChart() {
        if (this.canvas) {
            console.log('creating default chart');
            return {};
        }
    }

    populatePicker() {
        let picker = document.getElementById('chart-editor-picker');

        for (let pickerItem of this.pickerItems) {
            let pickerItemElement = document.createElement('div');
            pickerItemElement.id =
                'chart-editor-picker-item-' + pickerItem.color;
            pickerItemElement.classList.add('chart-editor-picker-item');
            pickerItemElement.style.backgroundColor = pickerItem.color;

            pickerItemElement.dataset.value = pickerItem.value.toString();
            pickerItemElement.dataset.length = pickerItem.length.toString();
            pickerItemElement.dataset.width = pickerItem.width.toString();
            pickerItemElement.dataset.height = pickerItem.height.toString();

            pickerItemElement.draggable = true;

            picker?.appendChild(pickerItemElement);
        }
    }

    addFloor(): Floor {
        console.log('adding floor');
        let floor = new Floor();
        this.floors.push(floor);
        return floor;
    }

    getCurrentFloor(): Floor {
        return this.currentFloor;
    }

    setHeightAt(x: number, y: number, height: number) {
        let cell = this.currentFloor.getCellAt(x, y);
        if (cell) {
            cell.height = height;
        } else {
            this.currentFloor.addCell(x, y, height);
        }
    }

    dragOverGrid(event: DragEvent) {
        if (event.preventDefault) {
            event.preventDefault();
        }
        console.log('dragging over grid, event: ', event);
    }

    dropOnGrid(event: DragEvent) {
        console.log('dropping on grid, event: ', event);
    }
}

export default ChartEditor;
