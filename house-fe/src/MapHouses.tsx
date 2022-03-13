import { DivIcon, Map, Point } from "leaflet";
import { MapContainer, Marker, TileLayer, Popup } from "react-leaflet";
import { useQuery } from "react-query";
import { HouseDTO } from "./List";
import HouseIcon from "./Mediamodifier-Design.svg";

import { useEffect, useRef, useState } from "react";
import { useSearchParams } from "react-router-dom";

export default function MapHouses() {
    const [map, setMap] = useState<Map>();
    const [searchParams, _] = useSearchParams();

    const houseId = searchParams.get('houseId')

    const housesQuery = useQuery<HouseDTO[]>('houses', () =>
        fetch('/api/houses').then(res =>
            res.json()
        )
    )

    if (housesQuery.isLoading) return (<div>Loading...</div>);
    if (housesQuery.error) return (<div>An error has occurred: </div>)
    if (!housesQuery.data) return (<div>OOOPS</div>)

    if (housesQuery.data.length === 0) {
        return <div>Empty</div>
    }

    const style = { height: '100%', width: '100%' }

    const withPosition = housesQuery.data.filter(d => d.lat && d.lng)

    let center: [number, number]
    if (houseId) {
        const d = withPosition.find(d => d.id === houseId)
        center = [d?.lat as number, d?.lng as number]
    } else {
        center = withPosition
            .reduce((s, d) => {
                return [s[0] + (d.lat as number), s[1] + (d.lng as number)]
            }, [0.0, 0.0]) as [number, number]
        center[0] = center[0] / withPosition.length
        center[1] = center[1] / withPosition.length

    }

    return (
        <MapContainer center={center} zoom={12} style={style} whenCreated={setMap}>
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            {
                withPosition.map(d => {
                    return <CustomMarker
                        key={d.id}
                        map={map}
                        isActive={houseId === d.id}
                        d={d}
                    />
                })
            }
        </MapContainer>
    )
}

const CustomMarker = ({ isActive, d, map }: { isActive?: boolean, d: HouseDTO, map: Map | undefined }) => {
    const [refReady, setRefReady] = useState(false);
    let popupRef = useRef<any>();

    const w = 30
    const h = 37

    useEffect(() => {
        if (map && isActive) {
            popupRef.current.openOn(map);
        }
    }, [isActive, refReady, map]);

    const divIcon = new DivIcon({
        html: `<img src="${HouseIcon}" class="vote-${d.vote}" width=${w} height=${h} alt="React Logo" />`,
        iconSize: new Point(w, h),
    })

    return (
        <Marker position={[d.lat as number, d.lng as number]} icon={divIcon}>
            <Popup ref={popupRef}>
                <p><a target="_blank" href={d.link}>{d.link}</a></p>
                <p>vote: {d.vote}</p>
                <p>{d.street}, {d.rooms_number} locali, {d.square_meters} mq</p>
                <p>{d.comment}</p>
            </Popup>
        </Marker>
    );
};