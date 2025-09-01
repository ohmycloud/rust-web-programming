import React, { useState } from "react";
import { createToDoItemCall } from "../api/create";

interface CreateItemFormProps {
  onBackResponse: (response: any) => void;
}

export const CreateToDoItem: React.FC<CreateItemFormProps> = (
  passBackResponse,
) => {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState<string>("");

  const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(e.target.value);
  };

  const createItem = async () => {
    await createToDoItemCall(title).then((response) => {
      setTitle("");
      if (response.data) {
        passBackResponse(response.data);
      } else if (response.error) {
        console.log(response);
        console.log(`Error ${response.status}: ${response.error}`);
      }
    });
  };
  return (
    <div className="inputContainer">
      <input
        type="text"
        id="name"
        placeholder="create to do item"
        value={title}
        onChange={handleTitleChange}
      />
      <button className="actionButton" id="create-button" onClick={createItem}>
        Create
      </button>
    </div>
  );
};
