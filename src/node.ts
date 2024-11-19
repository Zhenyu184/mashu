import { ClassicPreset } from 'rete';
const socket = new ClassicPreset.Socket('socket');
const baseHeight = 47;
const heightUnit = 36;
const uniformWidth = 200;

class HeadNode extends ClassicPreset.Node<{}, { success: ClassicPreset.Socket }> {
    height = baseHeight + heightUnit;
    width = uniformWidth;

    constructor() {
        super('HeadNode');
        this.addOutput('success', new ClassicPreset.Output(socket, 'Success'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

class EndNode extends ClassicPreset.Node<{ entry: ClassicPreset.Socket }, {}> {
    height = baseHeight + heightUnit;
    width = uniformWidth;

    constructor() {
        super('EndNode');
        this.addInput('entry', new ClassicPreset.Input(socket, 'Entry'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

class SleepNode extends ClassicPreset.Node<{ entry: ClassicPreset.Socket }, { success: ClassicPreset.Socket }> {
    height = baseHeight + heightUnit * 2;
    width = uniformWidth;

    constructor() {
        super('SleepNode');
        this.addInput('entry', new ClassicPreset.Input(socket, 'Entry'));
        this.addOutput('success', new ClassicPreset.Output(socket, 'Success'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

class TimingNode extends ClassicPreset.Node<{ entry: ClassicPreset.Socket }, { success: ClassicPreset.Socket }> {
    height = baseHeight + heightUnit * 2;
    width = uniformWidth;

    constructor() {
        super('TimingNode');
        this.addInput('entry', new ClassicPreset.Input(socket, 'Entry'));
        this.addOutput('success', new ClassicPreset.Output(socket, 'Success'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

class InitWebNode extends ClassicPreset.Node<
    { entry: ClassicPreset.Socket },
    { success: ClassicPreset.Socket; failure: ClassicPreset.Socket }
> {
    height = baseHeight + heightUnit * 3;
    width = uniformWidth;

    constructor() {
        super('InitWebNode');
        this.addInput('entry', new ClassicPreset.Input(socket, 'Entry'));
        this.addOutput('success', new ClassicPreset.Output(socket, 'Success'));
        this.addOutput('failure', new ClassicPreset.Output(socket, 'Failure'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

class OpenWebNode extends ClassicPreset.Node<
    { entry: ClassicPreset.Socket },
    { success: ClassicPreset.Socket; failure: ClassicPreset.Socket }
> {
    height = baseHeight + heightUnit * 3;
    width = uniformWidth;

    constructor() {
        super('OpenWebNode');
        this.addInput('entry', new ClassicPreset.Input(socket, 'Entry'));
        this.addOutput('success', new ClassicPreset.Output(socket, 'Success'));
        this.addOutput('failure', new ClassicPreset.Output(socket, 'Failure'));
    }

    data(): { value: any } {
        return { value: null };
    }
}

export { HeadNode, EndNode, SleepNode, TimingNode, InitWebNode, OpenWebNode };
