
import { QueryClient, QueryClientProvider } from 'react-query'
import ListHouses from './List';
import { SwitchInInsertMode } from './SwitchInInsertMode';
import { useState } from 'react';
import { InsertHouse } from './InsertHouse';
import { Grid, Theme } from '@material-ui/core';
import { makeStyles, createTheme, ThemeProvider } from '@material-ui/core/styles';
import MapHouses from './MapHouses';
import { HashRouter, Navigate, Route, Routes } from "react-router-dom";
import DetailHouse from './DetailHouse';

const useStyles = makeStyles((theme: Theme) => ({
  main: {
    paddingLeft: theme.spacing(1),
    paddingRight: theme.spacing(1),
    paddingTop: theme.spacing(2),
    height: '100%',
  }
}));

const queryClient = new QueryClient()

function App() {
  const classes = useStyles();

  const modes = [
    {
      fn: () => <InsertHouse />,
      path: '/insert',
    },
    {
      fn: () => <ListHouses />,
      path: '/list',
    },
    {
      fn: () => <MapHouses />,
      path: '/map'
    }
  ]

  return (
    <QueryClientProvider client={queryClient}>
      <HashRouter>
        <Grid
          container
          direction="column"
          justifyContent="center"
          alignItems="center"
          className={classes.main}
        >
          <SwitchInInsertMode />
          <Grid
            container
            direction="column"
            justifyContent="flex-start"
            alignItems="stretch"
            style={{ padding: '20px', gap: '20px', flex: 1 }}
          >
            <Routes>
              <Route key='insert' path='insert' element={<InsertHouse />} />
              <Route key='list' path='list' element={<ListHouses />} />
              <Route key='map' path='map' element={<MapHouses />} />
              <Route key='detail' path='houses/:houseId' element={<DetailHouse />} />
              <Route
                path="*"
                element={
                  <Navigate to="/list" />
                }
              />
            </Routes>
          </Grid>
        </Grid>
      </HashRouter>
    </QueryClientProvider >
  )
}

export default App;
