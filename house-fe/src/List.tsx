import { Grid, IconButton, ListItem, ListItemSecondaryAction, ListItemText } from '@material-ui/core';
import List from '@material-ui/core/List';
import { Delete } from '@material-ui/icons';
import ArrowForwardIosIcon from '@material-ui/icons/ArrowForwardIos';
import { HTMLAttributes } from 'react';
import RoomIcon from '@material-ui/icons/Room';
import { useMutation, useQuery } from 'react-query'
import { Link } from 'react-router-dom';
import { InsertHouseDto } from './InsertHouse'

export default function ListHouses() {
    const housesQuery = useQuery<HouseDTO[]>('houses', () =>
        fetch('/api/houses').then(res => res.json()),
        {
            refetchOnWindowFocus: false,
            refetchInterval: false,
            retry: false,
        }
    )

    const removeHouseMutation = useMutation((id: string) => {
        return fetch('/api/houses/' + id, {
            method: 'DELETE',
        })
    }, {
        onSuccess: () => housesQuery.refetch(),
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
                        <ListItem key={d.id} ContainerProps={{ 'data-testid': 'item-list-' + d.id } as HTMLAttributes<HTMLDivElement>}>
                            <ListItemText primary={`${d.street} (${d.zone})`} secondary={`${d.rooms_number} locali, ${d.square_meters}mq`} />
                            <ListItemSecondaryAction>

                                <Link
                                    style={{ marginLeft: '15px' }}
                                    to={`/map?houseId=${d.id}`}
                                >

                                    <IconButton edge="end" aria-label="Go to map">
                                        <RoomIcon />
                                    </IconButton>
                                </Link>

                                <IconButton style={{ marginLeft: '15px' }} edge="end" aria-label="Remove element" onClick={_ => removeHouseMutation.mutate(d.id)}>
                                    <Delete />
                                </IconButton>

                                <Link
                                    style={{ marginLeft: '15px' }}
                                    to={`/houses/${d.id}`}
                                >
                                    <IconButton edge="end" aria-label="Go to detail">
                                        <ArrowForwardIosIcon />
                                    </IconButton>
                                </Link>
                            </ListItemSecondaryAction>
                        </ListItem>
                    )
                })
            }
        </List >
    )
}

export interface HouseDTO extends InsertHouseDto {
    id: string
}