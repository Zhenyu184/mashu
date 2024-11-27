import { createRoot } from 'react-dom/client';
import { NodeEditor, GetSchemes, ClassicPreset } from 'rete';
import { AreaPlugin, AreaExtensions } from 'rete-area-plugin';
import { ConnectionPlugin, Presets as ConnectionPresets } from 'rete-connection-plugin';
import { ReactPlugin, Presets, ReactArea2D } from 'rete-react-plugin';
import { AutoArrangePlugin, Presets as ArrangePresets } from 'rete-auto-arrange-plugin';
import { DataflowEngine } from 'rete-engine';
import { ContextMenuExtra, ContextMenuPlugin, Presets as ContextMenuPresets } from 'rete-context-menu-plugin';

import contentManager from './content';
const curr = contentManager.getContent();

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
type ConnProps = Connection<Node, Node>;
type Schemes = GetSchemes<Node, ConnProps>;
type AreaExtra = ReactArea2D<any> | ContextMenuExtra;

export async function createEditor(container: HTMLElement) {
    const editor = new NodeEditor<Schemes>();
    const area = new AreaPlugin<Schemes, AreaExtra>(container);
    const connection = new ConnectionPlugin<Schemes, AreaExtra>();
    const render = new ReactPlugin<Schemes, AreaExtra>({ createRoot });
    const arrange = new AutoArrangePlugin<Schemes>();
    const engine = new DataflowEngine<Schemes>();

    // recalculate
    function recalculate() {
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
            ['Number', () => new NumberNode(0, recalculate)],
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

    // pipe change detection
    editor.addPipe((context) => {
        if (['connectioncreated', 'connectionremoved'].includes(context.type)) {
            recalculate();
        }
        return context;
    });

    // init layout
    const ct001 = new HeadNode();
    const op001 = new InitWebNode();
    const op002 = new OpenWebNode();
    const op003 = new InputStringNode();
    const op004 = new SummitNode();
    const ct003 = new SleepNode();
    const ct002 = new EndNode();

    await editor.addNode(ct001);
    await editor.addNode(op001);
    await editor.addNode(op002);
    await editor.addNode(op003);
    await editor.addNode(op004);
    await editor.addNode(ct003);
    await editor.addNode(ct002);

    const con1 = new Connection(ct001, 'success', op001, 'entry');
    const con2 = new Connection(op001, 'success', op002, 'entry');
    const con3 = new Connection(op002, 'success', op003, 'entry');
    const con4 = new Connection(op003, 'success', op004, 'entry');
    const con5 = new Connection(op004, 'success', ct003, 'entry');
    const con6 = new Connection(ct003, 'success', ct002, 'entry');

    await editor.addConnection(con1);
    await editor.addConnection(con2);
    await editor.addConnection(con3);
    await editor.addConnection(con4);
    await editor.addConnection(con5);
    await editor.addConnection(con6);

    await arrange.layout();
    AreaExtensions.zoomAt(area, editor.getNodes());

    return {
        destroy: () => area.destroy(),
    };
}
