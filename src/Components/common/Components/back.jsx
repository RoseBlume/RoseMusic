
import { handleMenuClick } from "../funcs";
export function Back() {
    return (
        <h2 onClick={handleMenuClick}>&larr; Back</h2>
    );
}