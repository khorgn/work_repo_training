
#if !defined(ROOMWITHABOMB_HPP)
#define ROOMWITHABOMB_HPP

#include "Room.hpp"

namespace abstractfactory::mazefactory
{
    class RoomWithABomb : public Room
    {
        public:
        RoomWithABomb(int roomNumber) : Room(roomNumber) {}
    };
}

#endif // ROOMWITHABOMB_HPP
