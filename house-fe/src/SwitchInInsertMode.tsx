import { Radio } from '@material-ui/core';
import { Add, List, Map } from '@material-ui/icons';

export enum Mode {
    Insert = 'insert',
    View = 'view',
    Map = 'map',
}

export function SwitchInInsertMode({ mode, onChangeMode }: SwitchInInsertModeProp) {
    const name = 'switch-mode'
    const style = { width: 53, height: 53 }
    return (
        <div>
            <Radio
                style={style}
                checked={mode === Mode.Insert}
                onChange={_ => onChangeMode(Mode.Insert)}
                value={Mode.Insert}
                name={name}
                inputProps={{ 'aria-label': Mode.Insert }}
                icon={<Add fontSize="small" />}
                checkedIcon={<Add fontSize="large" />}
            />
            <Radio
                style={style}
                checked={mode === Mode.View}
                onChange={_ => onChangeMode(Mode.View)}
                value={Mode.View}
                name={name}
                inputProps={{ 'aria-label': Mode.View }}
                icon={<List fontSize="small" />}
                checkedIcon={<List fontSize="large" />}
            />
            <Radio
                style={style}
                checked={mode === Mode.Map}
                onChange={_ => onChangeMode(Mode.Map)}
                value={Mode.Map}
                name={name}
                inputProps={{ 'aria-label': Mode.Map }}
                icon={<Map fontSize="small" />}
                checkedIcon={<Map fontSize="large" />}
            />
        </div>

    )
}

type OnChangeModeFunction = (newMode: Mode) => void;

interface SwitchInInsertModeProp {
    mode: Mode,
    onChangeMode: OnChangeModeFunction,
}
