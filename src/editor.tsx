import { createRoot } from 'react-dom/client';
import { NodeEditor, GetSchemes, ClassicPreset } from 'rete';
import { AreaPlugin, AreaExtensions } from 'rete-area-plugin';
import { ConnectionPlugin, Presets as ConnectionPresets } from 'rete-connection-plugin';
import { ReactPlugin, Presets, ReactArea2D } from 'rete-react-plugin';
import { AutoArrangePlugin, Presets as ArrangePresets } from 'rete-auto-arrange-plugin';
import { DataflowEngine } from 'rete-engine';
import { ContextMenuExtra, ContextMenuPlugin, Presets as ContextMenuPresets } from 'rete-context-menu-plugin';

const socket = new ClassicPreset.Socket('socket');

class RequestNode extends ClassicPreset.Node<
    {},
    { value: ClassicPreset.Socket },
    { text: ClassicPreset.InputControl<'text'> }
> {
    height = 135;
    width = 300;

    constructor(initial: string, change?: (value: string) => void) {
        super('RequestNode');
        this.addControl('text', new ClassicPreset.InputControl('text', { initial, change }));
        this.addOutput('value', new ClassicPreset.Output(socket, 'output name'));
    }

    data(): { value: string } {
        return { value: this.controls.text.value || '' };
    }

    job() {
        return;
    }
}

class NumberNode extends ClassicPreset.Node<
    {},
    { value: ClassicPreset.Socket },
    { value: ClassicPreset.InputControl<'number'> }
> {
    height = 135;
    width = 200;

    constructor(initial: number, change?: () => void) {
        super('Number');
        this.addControl('value', new ClassicPreset.InputControl('number', { initial, change }));
        this.addOutput('value', new ClassicPreset.Output(socket, 'Number'));
    }

    data(): { value: number } {
        return { value: this.controls.value.value || 0 };
    }
}

class AddNode extends ClassicPreset.Node<
    { left: ClassicPreset.Socket; right: ClassicPreset.Socket },
    { value: ClassicPreset.Socket },
    { value: ClassicPreset.InputControl<'number'> }
> {
    height = 220;
    width = 200;

    constructor(change?: () => void, private update?: (control: ClassicPreset.InputControl<'number'>) => void) {
        super('Add');
        const left = new ClassicPreset.Input(socket, 'Left');
        const right = new ClassicPreset.Input(socket, 'Right');

        left.addControl(new ClassicPreset.InputControl('number', { initial: 0, change }));
        right.addControl(new ClassicPreset.InputControl('number', { initial: 0, change }));

        this.addInput('left', left);
        this.addInput('right', right);
        this.addControl(
            'value',
            new ClassicPreset.InputControl('number', {
                readonly: true,
            })
        );
        this.addOutput('value', new ClassicPreset.Output(socket, 'Number'));
    }

    data(inputs: { left?: number[]; right?: number[] }): { value: number } {
        const leftControl = this.inputs.left?.control as ClassicPreset.InputControl<'number'>;
        const rightControl = this.inputs.right?.control as ClassicPreset.InputControl<'number'>;

        const { left, right } = inputs;
        const leftInputData = left ? left[0] : leftControl.value || 0;
        const rightInputData = right ? right[0] : rightControl.value || 0;

        const value = this.job(leftInputData, rightInputData);

        this.controls.value.setValue(value);
        if (this.update) {
            this.update(this.controls.value);
        }

        return { value };
    }

    job(a?: number, b?: number) {
        return (a || 0) + (b || 0);
    }
}

import {
    HeadNode,
    EndNode,
    SleepNode,
    TimingNode,
    InitWebNode,
    OpenWebNode,
    InputStringNode,
    PressButtonNode,
    SummitNode,
} from './node';
type Node =
    | NumberNode
    | AddNode
    | RequestNode
    | HeadNode
    | EndNode
    | SleepNode
    | TimingNode
    | InitWebNode
    | OpenWebNode
    | InputStringNode
    | PressButtonNode
    | SummitNode;
class Connection<A extends Node, B extends Node> extends ClassicPreset.Connection<A, B> {}
type ConnProps = Connection<NumberNode, AddNode> | Connection<AddNode, AddNode>;
type Schemes = GetSchemes<Node, ConnProps>;

type AreaExtra = ReactArea2D<any> | ContextMenuExtra;

export async function createEditor(container: HTMLElement) {
    const editor = new NodeEditor<Schemes>();
    const area = new AreaPlugin<Schemes, AreaExtra>(container);
    const connection = new ConnectionPlugin<Schemes, AreaExtra>();
    const render = new ReactPlugin<Schemes, AreaExtra>({ createRoot });
    const arrange = new AutoArrangePlugin<Schemes>();
    const engine = new DataflowEngine<Schemes>();

    function connectDetection() {
        engine.reset();
        editor
            .getNodes()
            .filter((n) => n instanceof AddNode)
            .forEach((n) => engine.fetch(n.id));
    }

    const contextMenu = new ContextMenuPlugin<Schemes>({
        items: ContextMenuPresets.classic.setup([
            ['HeadNode', () => new HeadNode()],
            ['EndNode', () => new EndNode()],
            ['SleepNode', () => new SleepNode()],
            ['TimingNode', () => new TimingNode()],
            ['InitWebNode', () => new InitWebNode()],
            ['OpenWebNode', () => new OpenWebNode()],
            ['Number', () => new NumberNode(0, connectDetection)],
            ['Add', () => new AddNode(connectDetection, (c) => area.update('control', c.id))],
        ]),
    });

    area.use(contextMenu);
    AreaExtensions.selectableNodes(area, AreaExtensions.selector(), {
        accumulating: AreaExtensions.accumulateOnCtrl(),
    });

    render.addPreset(Presets.contextMenu.setup());
    render.addPreset(Presets.classic.setup());
    connection.addPreset(ConnectionPresets.classic.setup());
    arrange.addPreset(ArrangePresets.classic.setup());

    editor.use(engine);
    editor.use(area);
    area.use(connection);
    area.use(render);
    area.use(arrange);

    AreaExtensions.simpleNodesOrder(area);
    AreaExtensions.showInputControl(area);

    editor.addPipe((context) => {
        if (['connectioncreated', 'connectionremoved'].includes(context.type)) {
            connectDetection();
        }
        return context;
    });

    // interface layout
    const a = new NumberNode(1, connectDetection);
    const b = new NumberNode(1, connectDetection);
    const c = new AddNode(connectDetection, (c) => area.update('control', c.id));
    const d = new RequestNode('https://www.rust-lang.org', connectDetection);

    const con1 = new Connection(a, 'value', c, 'left');
    const con2 = new Connection(b, 'value', c, 'right');

    await editor.addNode(a);
    await editor.addNode(b);
    await editor.addNode(c);
    await editor.addNode(d);

    await editor.addConnection(con1);
    await editor.addConnection(con2);

    await arrange.layout();
    AreaExtensions.zoomAt(area, editor.getNodes());

    return {
        destroy: () => area.destroy(),
    };
}
