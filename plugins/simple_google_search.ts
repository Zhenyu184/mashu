export const script = {
    name: 'head',
    type: 'control',
    continue: {
        success: {
            name: 'init_web',
            type: 'option',
            parameter: { driver_url: 'http://localhost:9515' },
            continue: {
                success: {
                    name: 'open_web',
                    type: 'option',
                    parameter: { web_url: 'www.google.com' },
                    continue: {
                        success: {
                            name: 'input_string',
                            type: 'option',
                            parameter: { component: '', input: 'red panda' },
                            continue: {
                                success: {
                                    name: 'press_button',
                                    type: 'option',
                                    parameter: { component: '' },
                                    continue: {
                                        success: {
                                            name: 'sleep',
                                            type: 'control',
                                            parameter: { time: 10000 },
                                            continue: {
                                                success: {
                                                    name: 'end',
                                                    type: 'control',
                                                },
                                            },
                                        },
                                        except: {
                                            name: 'end',
                                            type: 'control',
                                        },
                                    },
                                },
                                except: {
                                    name: 'end',
                                    type: 'control',
                                },
                            },
                        },
                        except: {
                            name: 'end',
                            type: 'control',
                        },
                    },
                },
                except: {
                    name: 'end',
                    type: 'control',
                },
            },
        },
    },
};
