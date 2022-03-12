
import { QueryClient, QueryClientProvider } from 'react-query'
import ListHouses from './List';
import { Mode, SwitchInInsertMode } from './SwitchInInsertMode';
import { useState } from 'react';
import { InsertHouse } from './InsertHouse';
import { Grid, Theme } from '@material-ui/core';
import { makeStyles, createTheme, ThemeProvider } from '@material-ui/core/styles';
import { green, purple } from '@material-ui/core/colors';
import MapHouses from './MapHouses';

const useStyles = makeStyles((theme: Theme) => ({
  main: {
    paddingLeft: theme.spacing(1),
    paddingRight: theme.spacing(1),
    height: '100%',
  }
}));

const queryClient = new QueryClient()

function App() {
  const [mode, setMode] = useState(Mode.View);

  const classes = useStyles();

  const map = {
    [Mode.Insert]: () => <InsertHouse />,
    [Mode.View]: () => <ListHouses />,
    [Mode.Map]: () => <MapHouses />,
  }

  return (
    <QueryClientProvider client={queryClient}>

      <Grid
        container
        direction="column"
        justifyContent="center"
        alignItems="center"
        className={classes.main}
      >
        <SwitchInInsertMode mode={mode} onChangeMode={setMode} />
        <Grid
          container
          direction="column"
          justifyContent="flex-start"
          alignItems="stretch"
          style={{ padding: '20px', gap: '20px', flex: 1 }}
        >{map[mode]()}
        </Grid>
      </Grid>



    </QueryClientProvider >
  )
}

export default App;
