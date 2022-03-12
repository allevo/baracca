import { Grid, IconButton, ListItem, ListItemSecondaryAction, ListItemText } from '@material-ui/core';
import List from '@material-ui/core/List';
import { Delete } from '@material-ui/icons';

import { useMutation, useQuery } from 'react-query'
import { InsertHouseDto } from './InsertHouse'

export default function ListHouses() {
    const housesQuery = useQuery<HouseDTO[]>('houses', () =>
        fetch('/api/houses').then(res =>
            res.json()
        )
    )

    const removeHouseMutation = useMutation((id: string) => {
        return fetch('/api/houses/' + id, {
            method: 'DELETE',
        })
    }, {
        onSuccess: () => housesQuery.refetch()
    })

    if (removeHouseMutation.isLoading) return (<div>Removing...</div>)

    if (housesQuery.isLoading) return (<div>Loading...</div>);
    if (housesQuery.error) return (<div>An error has occurred: </div>)
    if (!housesQuery.data) return (<div>OOOPS</div>)

    if (housesQuery.data.length === 0) {
        return <div>Empty</div>
    }

    return (
        <List>
            {
                housesQuery.data.map(d => {
                    return (
                        <ListItem key={d.id}>
                            <ListItemText primary={`${d.street} (${d.zone})`} secondary={`${d.rooms_number} locali, ${d.square_meters}mq`} />
                            <ListItemSecondaryAction>
                                <IconButton edge="end" aria-label="delete" onClick={_ => removeHouseMutation.mutate(d.id)}>
                                    <Delete />
                                </IconButton>
                            </ListItemSecondaryAction>
                        </ListItem>
                    )
                })
            }
        </List>
    )
}

export interface HouseDTO extends InsertHouseDto {
    id: string
}