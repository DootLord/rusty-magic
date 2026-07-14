import { useState } from "react";
import Card from "../card/card";
import "./collection.css";

interface CollectionProps {
    cards: ICard[];
}

function Collection({ cards }: CollectionProps) {
    const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);

    return (
        <div className="collection">
            {cards.map((card, index) => {
                const isHovered = hoveredIndex === index;
                const isPushedDown = hoveredIndex !== null && index > hoveredIndex;

                return (
                    <div
                        key={`${card.title}-${index}`}
                        className={[
                            "collection-card",
                            isHovered && "collection-card--hovered",
                            isPushedDown && "collection-card--pushed",
                        ].filter(Boolean).join(" ")}
                        style={{ zIndex: isHovered ? cards.length : cards.length - index }}
                        onMouseEnter={() => setHoveredIndex(index)}
                        onMouseLeave={() => setHoveredIndex(null)}
                    >
                        <Card card={card} />
                    </div>
                );
            })}
        </div>
    );
}

export default Collection;
