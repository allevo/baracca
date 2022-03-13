import { createStyles, makeStyles, Theme } from '@material-ui/core';
import { Add, List, Map } from '@material-ui/icons';
import { NavLink } from 'react-router-dom';


const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        nav: {
            paddingTop: theme.spacing(1),
            paddingBottom: theme.spacing(1),
            alignItems: 'center',
            textAlign: 'center',
            verticalAlign: 'middle',
            justifyContent: 'center',
        },
        navlink: {
            width: '53px',
            height: '53px',
            padding: theme.spacing(2),
            textAlign: 'center',
            alignItems: 'center',
            verticalAlign: 'middle',
            justifyContent: 'center',
        },
        notActive: {
            color: theme.palette.grey[400]
        },
        active: {
            color: theme.palette.secondary.main
        },
        icon: {
            fontSize: '1.25rem',
        },
    }),
);

export function SwitchInInsertMode() {
    const classes = useStyles();
    return (
        <nav className={classes.nav}>
            <NavLink
                className={classes.navlink}
                to={`insert`}
                key={'insert'}
            >
                {({ isActive }) => <Add fontSize={isActive ? 'large' : 'small'} className={isActive ? classes.active : classes.notActive} />}
            </NavLink>

            <NavLink
                className={classes.navlink}
                to={`list`}
                key={'list'}
            >
                {({ isActive }) => <List fontSize={isActive ? 'large' : 'small'} className={isActive ? classes.active : classes.notActive} />}
            </NavLink>

            <NavLink
                className={classes.navlink}
                to={`map`}
                key={'map'}
            >
                {({ isActive }) => <Map fontSize={isActive ? 'large' : 'small'} className={isActive ? classes.active : classes.notActive} />}
            </NavLink>
        </nav >

    )
}
