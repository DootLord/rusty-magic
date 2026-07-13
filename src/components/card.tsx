import "./card.css";

function Card({ card }: { card: ICard }) {
    return (
        <div className="card">
            <img className="card-image" src={card.url} alt={card.title} />
            <p className="card-title">{card.title}</p>
        </div>
    );
}

export default Card;
